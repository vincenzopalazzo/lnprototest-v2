// Server implement of lnprototest
//
// This server is implementing all the lightning rules
// for communicate between two nodes.
//
// For terminology used in this program, please visit
// <https://www.ibm.com/docs/en/aix/7.2?topic=protocol-tcpip-terminology>
//
// Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
package lnprototest

import (
	"bytes"
	"fmt"
	"net"
	"time"

	"github.com/btcsuite/btcd/btcec/v2"
	"github.com/btcsuite/btcd/wire"
	"github.com/lightningnetwork/lnd/brontide"
	"github.com/lightningnetwork/lnd/keychain"
	"github.com/lightningnetwork/lnd/lnwire"
)

type ProtoTestServer struct {
	Conn        *brontide.Conn
	PrivKeyECDH keychain.PrivKeyECDH
}

func MakeKeys() (*keychain.PrivKeyECDH, error) {
	remotePriv, err := btcec.NewPrivateKey()
	if err != nil {
		return nil, err
	}
	return &keychain.PrivKeyECDH{PrivKey: remotePriv}, nil
}

// / Make - Make a new ProtoTestServer with a random private key
func Make() (*ProtoTestServer, error) {
	xpriv, err := MakeKeys()
	if err != nil {
		return nil, err
	}
	return &ProtoTestServer{
		Conn:        nil,
		PrivKeyECDH: *xpriv,
	}, nil
}

// / Connect - Perform the connection with the peer with
// / the provided public key (aka nodeId) that it is listening
// / on the specified port.
func (self *ProtoTestServer) Connect(nodeId string, port uint32, network wire.BitcoinNet) (*bytes.Buffer, error) {
	pubkey, err := StringToPubKey(nodeId)
	if err != nil {
		return nil, err
	}
	hostname := fmt.Sprintf("127.0.0.1:%d", port)
	addr, err := net.ResolveTCPAddr("tcp", hostname)
	if err != nil {
		return nil, err
	}
	wireaddr := lnwire.NetAddress{
		Address:     addr,
		IdentityKey: pubkey,
		ChainNet:    network,
	}
	conn, err := brontide.Dial(&self.PrivKeyECDH, &wireaddr, time.Second*3, net.DialTimeout)
	if err != nil {
		return nil, err
	}
	self.Conn = conn
	if err := self.Conn.SetDeadline(time.Now().Add(time.Second)); err != nil {
		return nil, err
	}
	return self.Receive()
}

// / Send - Send an message to the connection
func (self *ProtoTestServer) Send(buff *bytes.Buffer) error {
	size, err := self.Conn.Write(buff.Bytes())
	if err != nil {
		return err
	}
	if size == 0 {
		return fmt.Errorf("No message to flush")
	}
	return nil
}

// / Send - Wait a message from the connection, usually this is
// / an answer from a previous send message
func (self *ProtoTestServer) Receive() (*bytes.Buffer, error) {
	buff, err := self.Conn.ReadNextMessage()
	if err != nil {
		return nil, err
	}
	return bytes.NewBuffer(buff), nil
}

// / Destroy - Stop the connecto for lnprototest
func (self *ProtoTestServer) Destroy() {
	if self.Conn == nil {
		return
	}
	(*self.Conn).Close()
}
