#include "codec_source/Wirehair.hpp"
#include "Clock.hpp"
using namespace cat;

#include <iostream>
#include <fstream>
using namespace std;	

static Clock m_clock;

#define TRIALS 10

int main()
{
	m_clock.OnInitialize();

	int block_count = 32768;
	int block_bytes = 1;
	int message_bytes = block_bytes * block_count;
	u8 *message = new u8[message_bytes];
	u8 *message_out = new u8[message_bytes];
	u8 *block = new u8[block_bytes];

	for (int ii = 0; ii < message_bytes; ++ii)
	{
		message[ii] = ii;
	}

	g_d_seed = 7;
	g_p_seed = 0;

	wirehair::Encoder encoder;
	u64 successes = 0, trials = 0;
	//double time_sum = 0;

	for (;; ++g_p_seed)
	{
		++trials;

		double start = m_clock.usec();
		wirehair::Result r = encoder.BeginEncode(message, message_bytes, block_bytes);
		double end = m_clock.usec();

		if (r)
		{
			//if (trials % 10000 == 0)
			cout << "-- FAIL! encoder.BeginEncode error " << wirehair::GetResultString(r) << ".  Success rate = " << successes / (double)trials << endl;
			//cin.get();
			//return 1;
		}
		else
		{
			//time_sum += end - start;
			//cout << time_sum / successes << endl;

			++successes;
			//if (trials % 10000 == 0)
			cout << ">> OKAY! encoder.BeginEncode in " << end - start << " usec, " << message_bytes / (end - start) << " MB/s with seeds " << g_d_seed << " and " << g_p_seed << ".  Success rate = " << successes / (double)trials << endl;
			//cin.get();
			//break;
		}
	}

	CatsChoice prng;
	cat::wirehair::Decoder decoder;

	u32 overhead_sum = 0, overhead_trials = 0;
	u32 drop_seed = 0;
	for (;;)
	{
		int blocks_needed = 0;

		wirehair::Result s = decoder.BeginDecode(message_out, message_bytes, block_bytes);
		if (s)
		{
			cout << "-- FAIL! decoder.BeginEncode error " << wirehair::GetResultString(s) << endl;
			cin.get();
			return 1;
		}

		prng.Initialize(drop_seed);
		for (u32 id = 0;; ++id)
		{
			if (id == 0) continue;
			//if (prng.Next() & 1) continue;
			encoder.Encode(id, block);

			++blocks_needed;
			double start = m_clock.usec();
			wirehair::Result r = decoder.Decode(id, block);
			double end = m_clock.usec();

			if (r != wirehair::R_MORE_BLOCKS)
			{
				if (r == wirehair::R_WIN)
				{
					u32 overhead = blocks_needed - decoder.BlockCount();
					overhead_sum += overhead;
					++overhead_trials;

					cout << ">> OKAY! decoder.Decode in " << end - start << " usec, " << message_bytes / (end - start) << " MB/s after " << overhead << " extra blocks.  Average extra = " << overhead_sum / (double)overhead_trials << ". Seed = " << drop_seed << endl;

					if (!memcmp(message, message_out, message_bytes))
					{
						//cout << "Match!" << endl;
					}
					else
					{
						cout << "FAAAAAIL!" << endl;

						for (int ii = 0; ii < message_bytes; ++ii)
							cout << (int)message_out[ii] << endl;

						cin.get();
					}
				}
				else
				{
					cout << "-- FAIL! decoder.Decode error " << wirehair::GetResultString(r) << endl;

					cin.get();
				}

				//cin.get();
				break;
			}
		}

		++drop_seed;
	}

	m_clock.OnFinalize();

	return 0;
}
