use cxx::let_cxx_string;

use crate::ffi::*;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {

        include!("nCore.h");
        include!("nWAV.h");
        include!("fftw_interface.h");
        include!("nFFT.h");
        include!("nFillFFT.h");
        include!("nSpreading.h");
        include!("nInitialise.h");
        include!("nSGNS.h");
        include!("nParameter.h");
        include!("nProcess.h");
        include!("nRemoveBits.h");
        include!("nOutput.h");
        include!("nShiftBlocks.h");

        fn nCore_Init();
        fn set_global_blocks_processed(value: i32);
        fn audio_data_size_next() -> i32;
        fn parameters_is_merging() -> bool;
        fn global_codec_block_size() -> i32;
        fn global_last_codec_block() -> bool;
        fn set_global_last_codec_block(value: bool);
        fn global_first_codec_block() -> bool;
        fn set_global_first_codec_block(value: bool);
        fn parameters_correction() -> bool;
        fn set_parameters_correction(value: bool);
        fn lossyWAVError(lwe_string: &CxxString, lwe_value: i32);

        fn nWAV_Init();
        fn writeNextBTRDcodecblock() -> bool;
        fn readNextNextCodecBlock() -> bool;
        fn writeNextCORRcodecblock() -> bool;
        fn openWavIO() -> bool;
        fn closeWavIO() -> bool;
        fn MergeFiles();
        fn nWAV_Cleanup();

        fn FFTW_Initialised() -> bool;
        fn FFTW_Initialise() -> bool;
        fn FFTW_Cleanup();

        fn nFFT_Init(desired_max_fft_bit_length: i32);
        fn nFFT_Cleanup();

        fn nFillFFT_Init();
        fn nFillFFT_Cleanup();

        fn nSpreading_Init();
        fn nSpreading_Cleanup();

        fn nSGNS_Cleanup();

        fn nCheck_Switches();

        fn nParameter_Init(args: Vec<String>);
        fn nParameter_Cleanup();

        fn nProcess_Init();
        fn Process_This_Codec_Block();
        fn nProcess_Cleanup();

        fn nInitial_Setup();

        fn nRemoveBits_Init();

        fn nOutput_Init();
        fn write_cleanup();

        fn Shift_Codec_Blocks();
    }
}

struct Init {}

impl Init {
    fn Init() -> Self {
        nCore_Init();

        nWAV_Init();

        FFTW_Initialise();

        if !FFTW_Initialised() {
            const MAX_BLOCK_BITS: i32 = 12;
            static MAX_FFT_BIT_LENGTH: i32 = MAX_BLOCK_BITS + 1;
            nFFT_Init(MAX_FFT_BIT_LENGTH);
        }

        Init {}
    }
}

impl Drop for Init {
    fn drop(&mut self) {
        nWAV_Cleanup();

        nFillFFT_Cleanup();

        nSpreading_Cleanup();

        nSGNS_Cleanup();

        nParameter_Cleanup();

        if FFTW_Initialised() {
            FFTW_Cleanup();
        } else {
            nFFT_Cleanup();
        }

        nProcess_Cleanup();
    }
}

fn main() {
    let init = Init::Init();

    let args = std::env::args().collect::<Vec<String>>();
    nParameter_Init(args);

    nCheck_Switches();

    if parameters_is_merging() {
        MergeFiles();
    } else {
        if !openWavIO() {
            let_cxx_string!(error_message = "Error initialising wavIO unit.");
            lossyWAVError(&error_message, 0x11);
        }

        if global_codec_block_size() == 0 {
            let_cxx_string!(error_message = "Error initialising wavIO unit.");
            lossyWAVError(&error_message, 0x11);
        }

        nInitial_Setup();

        nSpreading_Init();

        nProcess_Init();

        nFillFFT_Init(); // dependent on Codec_Block_Size.

        nRemoveBits_Init(); // bitdepth and samplerate dependent.

        nOutput_Init();

        if !readNextNextCodecBlock() {
            let_cxx_string!(error_message = "Error reading from input file.");
            lossyWAVError(&error_message, 0x21);
        }

        set_global_blocks_processed(0);

        //==========================================================================
        // Main processing loop.
        //==========================================================================
        while audio_data_size_next() > 0 {
            set_global_last_codec_block(audio_data_size_next() == 0);

            set_global_first_codec_block(audio_data_size_next() == 0);

            Shift_Codec_Blocks();

            readNextNextCodecBlock();

            Process_This_Codec_Block();

            if !writeNextBTRDcodecblock() {
                let_cxx_string!(error_message = "Error writing to output file.");
                lossyWAVError(&error_message, 0x21);
            }

            if parameters_correction() {
                if !writeNextCORRcodecblock() {
                    let_cxx_string!(error_message = "Error writing to correction file.");
                    lossyWAVError(&error_message, 0x22);
                }
            }
        }

        if !closeWavIO() {
            let_cxx_string!(error_message = "Error closing wavIO unit.");
            lossyWAVError(&error_message, 0x11);
        }

        write_cleanup();
    }

    println!("Hello, world!");
}
