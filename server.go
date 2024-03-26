package main

import (
	"fmt"
	"net"
	"net/rpc"
	"net/rpc/jsonrpc"
	"os"

	"github.com/vincenzopalazzo/lnprototest-v2/lnprototest"
)

var lnprototestServer *lnprototest.ProtoTestServer = nil

type Server struct {
	dataDir string
}

func Make(datadir string) (*Server, error) {
	return &Server{
		dataDir: datadir,
	}, nil
}

func (self *Server) RegisterRPCs() error {
	if err := rpc.Register(Connect); err != nil {
		return nil
	}
	return nil
}

func (self *Server) Listen() error {
	if err := self.RegisterRPCs(); err != nil {
		return err
	}

	unixPath := fmt.Sprintf("%s/lnprototest.sock", self.dataDir)
	listener, err := net.Listen("unix", unixPath)
	if err != nil {
		return err
	}
	defer listener.Close()
	defer os.Remove(unixPath)

	// Init the lnprototest code
	lnprototestServer, err = lnprototest.Make()
	if err != nil {
		return err
	}

	for {
		conn, err := listener.Accept()
		if err != nil {
			continue
		}
		go jsonrpc.ServeConn(conn)
	}
}
