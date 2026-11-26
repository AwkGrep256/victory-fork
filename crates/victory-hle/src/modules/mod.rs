//! Implementations, one module per area of the console API.
//!
//! Adding an API: write an `extern "system"` function named exactly like
//! the import, then list it with `hle_export!` in the module's `module()`.
//! Run `victory census` over your titles to see what is still missing.
//!
//! Planned areas (driven by census results, not by completeness):
//! `file` (package paths), `thread` (TLS callbacks for title threads),
//! `xaudio2`, `xstore` stubs.

#[cfg(windows)]
pub mod apu;
pub mod audio;
pub mod d3d12x;
pub mod display;
pub mod dsp;
pub mod fidelityfx;
#[cfg(windows)]
pub mod dstorage;
#[cfg(windows)]
pub mod ffx;
#[cfg(windows)]
pub mod files;
pub mod gameinput;
pub mod kernel;
#[cfg(windows)]
pub mod lifecycle;
#[cfg(windows)]
pub mod media;
#[cfg(windows)]
pub mod net;
pub mod speech;
pub mod storage;
pub mod xasync;
pub mod xg;
pub mod xgame;
pub mod xgameruntime;
pub mod xgamesave;
pub mod xgameui;
#[cfg(windows)]
pub mod xmem;
pub mod xnetworking;
pub mod xpackage;
pub mod xstore;
pub mod xsystem;
pub mod xuser;

use crate::HleModule;

pub(crate) fn all() -> Vec<HleModule> {
    let mut all = vec![
        kernel::module(),
        xgameruntime::module(),
        gameinput::module(),
        d3d12x::module(),
        audio::module(),
        dsp::module(),
        xg::module(),
    ];
    #[cfg(windows)]
    all.push(apu::module());
    #[cfg(windows)]
    all.push(media::module());
    #[cfg(windows)]
    all.push(dstorage::module());
    #[cfg(windows)]
    all.push(files::module());
    #[cfg(windows)]
    all.push(lifecycle::module());
    #[cfg(windows)]
    all.push(net::module());
    #[cfg(windows)]
    all.push(xmem::module());
    #[cfg(windows)]
    all.push(xsystem::module());
    #[cfg(windows)]
    all.push(if victory_gpu::ffx::enabled() {
        ffx::module()
    } else {
        fidelityfx::module()
    });
    #[cfg(not(windows))]
    all.push(fidelityfx::module());
    all
}

#[cfg(all(test, windows))]
mod tests {
    #[test]
    fn only_one_module_serves_the_fidelityfx_loader() {
        let modules = super::all();
        let mut providers = modules
            .iter()
            .filter(|module| module.serves("amd_fidelityfx_loader_xbox.dll"));
        let provider = providers.next().expect("FidelityFX provider");
        assert!(providers.next().is_none());
        let expected = if std::env::var_os("VICTORY_FFX_DIR").is_some() {
            "ffx"
        } else {
            "fidelityfx"
        };
        assert_eq!(provider.name, expected);
    }
}
