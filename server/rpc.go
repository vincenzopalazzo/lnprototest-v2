package server

import (
	"bytes"
	"encoding/hex"

	"github.com/btcsuite/btcd/wire"
)

type ConnectRPC struct {
	NodeId string
	port   uint64
}

func (_ *ConnectRPC) Call(request *ConnectRPC, response *ConnectRPC) error {
	if err := lnprototestServer.Connect(request.NodeId, uint32(request.port), wire.SimNet); err != nil {
		return err
	}
	response.NodeId = request.NodeId
	response.port = request.port
	return nil
}

type SendRPC struct {
	msg string
}

func (_ *SendRPC) Call(request *SendRPC, response *SendRPC) error {
	buff, err := hex.DecodeString(request.msg)
	if err != nil {
		return err
	}

	if err := lnprototestServer.Send(bytes.NewBuffer(buff)); err != nil {
		return err
	}

	buffResp, err := lnprototestServer.Receive()
	if err != nil {
		return err
	}

	response.msg = hex.EncodeToString(buffResp.Bytes())
	return nil
}
