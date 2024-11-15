package server

import (
	"context"
	"errors"
	"fmt"
	"net"
	"os"

	"github.com/charmbracelet/log"
	"github.com/sourcegraph/jsonrpc2"

	"github.com/vincenzopalazzo/lnprototest-v2/lnprototest"
)

var lnprototestServer *lnprototest.ProtoTestServer = nil

type Server struct {
	dataDir string
	context context.Context
	// queue of *jsonrpc2.Conn
	queue []*jsonrpc2.Conn
}

func Make(datadir string) (*Server, error) {
	return &Server{
		dataDir: datadir,
		context: context.Background(),
		queue:   make([]*jsonrpc2.Conn, 0),
	}, nil
}

func (self *Server) Listen() error {
	unixPath := fmt.Sprintf("%s/lnprototest.sock", self.dataDir)
	if _, err := os.Stat(unixPath); !errors.Is(err, os.ErrNotExist) {
		os.Remove(unixPath)
	}

	listener, err := net.Listen("unix", unixPath)
	if err != nil {
		return err
	}
	defer listener.Close()
	defer os.Remove(unixPath)

	log.Infof("Listening on %s", unixPath)
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

		_ = jsonrpc2.NewConn(self.context, jsonrpc2.NewPlainObjectStream(conn), &RPCHandler{})
	}
}
