// / Lightning Network protocol integration tests
// /
// / Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
package main

import (
	"fmt"
	"os"

	"github.com/akamensky/argparse"
)

type CmdParser struct {
	parser  *argparse.Parser
	dataDir *string
	network *string
}

func (self *CmdParser) Parse(args []string) error {
	return self.parser.Parse(args)
}

func (self *CmdParser) Usage(error error) string {
	return self.parser.Usage(error)
}

func buildCmdParser() (*CmdParser, error) {
	parser := argparse.NewParser("lnprototestd", "Lightning Network protocol test Deamon (I am a simple proxy)")
	home, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}
	dataDir := parser.String("d", "data-dir", &argparse.Options{Required: false, Default: home, Help: "data directory for ocean market deamon"})
	_ = parser.Int("p", "port", &argparse.Options{Required: false, Default: 9090, Help: "Default port where the liquidity provider will listen"})
	_ = parser.String("a", "address", &argparse.Options{Required: false, Default: "127.0.0.1", Help: "Host where the liquidity provider will listen about new connection"})
	network := parser.String("n", "network", &argparse.Options{Required: false, Default: "testnet", Help: "The Bitcoin Network"})
	return &CmdParser{parser: parser, dataDir: dataDir, network: network}, nil
}

func main() {
	parser, err := buildCmdParser()
	if err != nil {
		panic(fmt.Errorf("%s", err))
	}

	if err := parser.Parse(os.Args); err != nil {
		panic(parser.Usage(err))
	}
}
