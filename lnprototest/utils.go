package lnprototest

import (
	"encoding/hex"

	"github.com/btcsuite/btcd/btcec/v2"
)

// / StringToPubKey - Convert a pub key in string format
// / in a PublicKye type
func StringToPubKey(pubkey string) (*btcec.PublicKey, error) {
	bytes, err := hex.DecodeString(pubkey)
	if err != nil {
		return nil, err
	}
	key, err := btcec.ParsePubKey(bytes)
	if err != nil {
		return nil, err
	}
	return key, nil
}
