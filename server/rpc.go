package server

import (
	"bytes"
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"

	"github.com/btcsuite/btcd/wire"
	"github.com/charmbracelet/log"
	"github.com/sourcegraph/jsonrpc2"
)

type ConnectRPC struct {
	NodeId string
	Port   uint32
	// Afeter the connection we should have a
	// init message from the node
	Msg string
}

func ConnectCall(request *json.RawMessage, response *json.RawMessage) error {
	var connect ConnectRPC
	log.Infof("%s", string(*request))
	if err := json.Unmarshal(*request, &connect); err != nil {
		return nil
	}
	log.Infof("Connect to %s:%d", connect.NodeId, connect.Port)
	resp, err := lnprototestServer.Connect(connect.NodeId, connect.Port, wire.SimNet)
	if err != nil {
		return err
	}
	connect.Msg = hex.EncodeToString(resp.Bytes())
	log.Infof("Connected response %s", connect.Msg)
	*response, err = json.Marshal(connect)
	if err != nil {
		return err
	}
	return nil
}

type SendRPC struct {
	Msg string
}

func SendCall(request *json.RawMessage, response *json.RawMessage) error {
	var sendCall SendRPC
	if err := json.Unmarshal(*request, &sendCall); err != nil {
		return nil
	}

	log.Debugf("Trying to send %s", sendCall.Msg)
	buff, err := hex.DecodeString(sendCall.Msg)
	if err != nil {
		return err
	}
	if err := lnprototestServer.Send(bytes.NewBuffer(buff)); err != nil {
		log.Errorf("Error sending the message %s", err)
		return err
	}
	log.Infof("Message sent %s", sendCall.Msg)
	buffResp, err := lnprototestServer.Receive()
	if err != nil {
		log.Errorf("Error receiving the message %s", err)
		return err
	}

	if buffResp == nil {
		return fmt.Errorf("empty answer from the node")
	}

	sendCall.Msg = hex.EncodeToString(buffResp.Bytes())
	log.Infof("Message received %s", sendCall.Msg)
	*response, err = json.Marshal(sendCall)
	return err
}

type RPCHandler struct{}

// Handle implements the jsonrpc2.Handler interface.
func (h *RPCHandler) Handle(ctx context.Context, c *jsonrpc2.Conn, r *jsonrpc2.Request) {
	log.Debugf("Received request: %s", r.Method)
	switch r.Method {
	case "connect":
		var response json.RawMessage
		if err := ConnectCall(r.Params, &response); err != nil {
			if err := c.ReplyWithError(ctx, r.ID, &jsonrpc2.Error{
				Code:    -1,
				Message: fmt.Sprintf("%s", err),
				Data:    nil,
			}); err != nil {
				return
			}
		}
		if err := c.Reply(ctx, r.ID, response); err != nil {
			return
		}
	case "send":
		var response json.RawMessage
		if err := SendCall(r.Params, &response); err != nil {
			if err := c.ReplyWithError(ctx, r.ID, &jsonrpc2.Error{
				Code:    -1,
				Message: fmt.Sprintf("%s", err),
				Data:    nil,
			}); err != nil {
				return
			}
		}
	default:
		err := &jsonrpc2.Error{Code: jsonrpc2.CodeMethodNotFound, Message: "Method not found"}
		if err := c.ReplyWithError(ctx, r.ID, err); err != nil {
			return
		}
	}
}
