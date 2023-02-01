package main

import (
	"bytes"
	"encoding/binary"
	"io"
	"io/ioutil"
	"os"
	"testing"

	"github.com/marcopolo/proto3-and-2/go/m/v2/pb/p2"
	"github.com/marcopolo/proto3-and-2/go/m/v2/pb/p3"
	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/proto"
)

const HOP_FILE = "../randomHopMsgs.bin"
const STOP_FILE = "../randomStopMsgs.bin"

func TestDecodingHop(t *testing.T) {
	f, err := os.OpenFile(HOP_FILE, os.O_RDONLY, 0644)
	require.NoError(t, err)
	data, err := ioutil.ReadAll(f)
	buf := bytes.NewBuffer(nil)
	require.NoError(t, err)
	r := bytes.NewReader(data)
	for r.Len() > 0 {
		size, err := binary.ReadUvarint(r)
		require.NoError(t, err)

		buf.Reset()
		buf.Grow(int(size))
		b := buf.Bytes()[0:size]

		_, err = io.ReadFull(r, b)
		require.NoError(t, err)

		hop2 := p2.HopMessage{}
		err = proto.Unmarshal(b, &hop2)
		require.NoError(t, err)

		hop3 := p3.HopMessage{}
		err = proto.Unmarshal(b, &hop3)
		require.NoError(t, err)

		if hop2.Type == nil {
			t.Fatal("type is nil")
		}

		if int32(*hop3.Type) != int32(*hop2.Type) {
			t.Fatal("type mismatch")
		}

		if hop3.Peer != nil && hop2.Peer != nil {
			require.Equal(t, hop2.Peer.Addrs, hop3.Peer.Addrs)
			require.Equal(t, hop2.Peer.Id, hop3.Peer.Id)
		} else if hop3.Peer != nil || hop2.Peer != nil {
			t.Fatal("peer mismatch")
		}

		if hop3.Reservation != nil && hop2.Reservation != nil {
			require.Equal(t, *hop2.Reservation.Expire, *hop3.Reservation.Expire)
			require.Equal(t, hop2.Reservation.Addrs, hop3.Reservation.Addrs)
			require.Equal(t, hop2.Reservation.Voucher, hop3.Reservation.Voucher)
		} else if hop3.Reservation != nil || hop2.Reservation != nil {
			t.Fatal("Reservation mismatch")
		}

		if hop3.Limit != nil && hop2.Limit != nil {
			require.Equal(t, hop2.Limit.Data, hop3.Limit.Data)
			require.Equal(t, hop2.Limit.Duration, hop3.Limit.Duration)
		} else if hop3.Limit != nil || hop2.Limit != nil {
			t.Fatal("Limit mismatch")
		}

		if hop3.Status != nil && hop2.Status != nil {
			require.Equal(t, int32(*hop2.Status), int32(*hop3.Status))
		} else if hop3.Status != nil || hop2.Status != nil {
			t.Fatal("Status mismatch")
		}
	}
}

func TestDecodingStop(t *testing.T) {
	f, err := os.OpenFile(STOP_FILE, os.O_RDONLY, 0644)
	require.NoError(t, err)
	data, err := ioutil.ReadAll(f)
	buf := bytes.NewBuffer(nil)
	require.NoError(t, err)
	r := bytes.NewReader(data)
	for r.Len() > 0 {
		size, err := binary.ReadUvarint(r)
		require.NoError(t, err)

		buf.Reset()
		buf.Grow(int(size))
		b := buf.Bytes()[0:size]

		_, err = io.ReadFull(r, b)
		require.NoError(t, err)

		stop2 := p2.StopMessage{}
		err = proto.Unmarshal(b, &stop2)
		require.NoError(t, err)

		stop3 := p3.StopMessage{}
		err = proto.Unmarshal(b, &stop3)
		require.NoError(t, err)

		if stop2.Type == nil {
			t.Fatal("type is nil")
		}

		if int32(*stop3.Type) != int32(*stop2.Type) {
			t.Fatal("type mismatch")
		}

		if stop3.Peer != nil && stop2.Peer != nil {
			require.Equal(t, stop2.Peer.Addrs, stop3.Peer.Addrs)
			require.Equal(t, stop2.Peer.Id, stop3.Peer.Id)
		} else if stop3.Peer != nil || stop2.Peer != nil {
			t.Fatal("peer mismatch")
		}

		if stop3.Limit != nil && stop2.Limit != nil {
			require.Equal(t, stop2.Limit.Data, stop3.Limit.Data)
			require.Equal(t, stop2.Limit.Duration, stop3.Limit.Duration)
		} else if stop3.Limit != nil || stop2.Limit != nil {
			t.Fatal("Limit mismatch")
		}

		if stop3.Status != nil && stop2.Status != nil {
			require.Equal(t, int32(*stop2.Status), int32(*stop3.Status))
		} else if stop3.Status != nil || stop2.Status != nil {
			t.Fatal("Status mismatch")
		}
	}
}
