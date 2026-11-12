# License status

The supplied ModernBASIC attachments do not contain a project-level license.
No new license is inferred or granted by this repair. Before public or
commercial distribution, the project owner should add an explicit license and
confirm ownership or permission for every retained source and asset.

The vendored SoundFont synthesizer derives from RustySynth/MeltySynth. Its MIT
notice is preserved at:

`THIRD_PARTY_LICENSES/RustySynth-LICENSE.txt`

Seven SoundFont banks generated from this project's Ruststrument models are
tracked under `soundfonts/`; the runtime maps both `soundfont/` and
`soundfonts/` BASIC prefixes to that pack. They are project-generated assets,
not the optional third-party banks described below. Because the project still
has no project-level license, their presence does not itself grant
redistribution rights. Any newly tracked third-party bank must carry its own
license and attribution.

The optional `install-soundfont.sh` helper downloads MuseScore General and its
MIT license directly from MuseScore's official mirror. The downloaded bank and
license remain separate from this source release.

The CPU table backend bundles an adapted Polars core 0.36.2 and the pinned
sources listed in `src/vendor/analytics-provenance.json`. All supplied upstream
licenses remain with their active source. Intentional changes to worker limits,
cleanup, standard strings and float parsing are recorded in
`src/vendor/analytics-patches.patch`. The former smartstring, static_assertions
and fast-float dependencies are removed from the active graph and source bundle;
their source history and notices remain in protected checkpoint archives.
See `src/vendor/analytics-NOTICES.md`. Separately distributed exported
applications still need the notices appropriate to their exact linked versions;
source changes do not alter the obligations of earlier exported binaries.
