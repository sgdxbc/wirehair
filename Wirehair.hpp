/*
	Copyright (c) 2012 Christopher A. Taylor.  All rights reserved.

	Redistribution and use in source and binary forms, with or without
	modification, are permitted provided that the following conditions are met:

	* Redistributions of source code must retain the above copyright notice,
	  this list of conditions and the following disclaimer.
	* Redistributions in binary form must reproduce the above copyright notice,
	  this list of conditions and the following disclaimer in the documentation
	  and/or other materials provided with the distribution.
	* Neither the name of LibCat nor the names of its contributors may be used
	  to endorse or promote products derived from this software without
	  specific prior written permission.

	THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
	AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
	IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
	ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
	LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
	CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
	SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
	INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
	CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
	ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
	POSSIBILITY OF SUCH DAMAGE.
*/

#ifndef CAT_WIREHAIR_HPP
#define CAT_WIREHAIR_HPP

#include "WirehairDetails.hpp"

namespace cat {

namespace wirehair {


/*
	Wirehair Streaming Forward Error Correction

		Wirehair is an FEC codec used to improve reliability of data sent
	over a Binary Erasure Channel (BEC) such as satellite Internet or WiFi.
	The data to transmit over the BEC is encoded into equal-sized blocks.
	When enough blocks are received at the other end of the channel, then
	the original data can be recovered.
*/

class Encoder
{
public:
	// Attempt to initialize the encoder for a given message
	// Returns false on initialization failure
	bool Initialize(const void *message_in, int message_bytes, int block_bytes);

	// Generate a block of size block_bytes specified during initialization
	void Generate(void *block);
};

class Decoder
{
public:
	// Attempt to initialize the decoder
	// Decoder will write to the given buffer once decoding completes
	// Returns false on initialization failure
	bool Initialize(void *message_out, int message_bytes, int block_bytes);

	// Decode the provided block of size block_bytes specified during initialization
	bool Decode(void *block);
};


} // namespace wirehair

} // namespace cat

#endif // CAT_WIREHAIR_HPP