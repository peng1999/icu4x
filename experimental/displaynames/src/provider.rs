// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
#![warn(unused_imports)]
//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroMap;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as displaynames;
        pub use icu_locid_transform as locid_transform;
    }
    icu_displaynames_data::make_provider!(Baked);
    icu_displaynames_data::impl_displaynames_languages_v1!(Baked);
    icu_displaynames_data::impl_displaynames_locales_v1!(Baked);
    icu_displaynames_data::impl_displaynames_regions_v1!(Baked);
    icu_displaynames_data::impl_displaynames_scripts_v1!(Baked);
    icu_displaynames_data::impl_displaynames_variants_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[
    LanguageDisplayNamesV1Marker::KEY,
    LocaleDisplayNamesV1Marker::KEY,
    RegionDisplayNamesV1Marker::KEY,
    ScriptDisplayNamesV1Marker::KEY,
    VariantDisplayNamesV1Marker::KEY,
];

// We use raw TinyAsciiStrs for map keys, as we then don't have to
// validate them as subtags on deserialization. Map lookup can be
// done even if they are not valid tags (an invalid key will just
// become inaccessible).
type UnvalidatedRegion = UnvalidatedTinyAsciiStr<3>;
type UnvalidatedLanguage = UnvalidatedTinyAsciiStr<3>;
type UnvalidatedScript = UnvalidatedTinyAsciiStr<4>;
type UnvalidatedLocale = UnvalidatedStr;
type UnvalidatedVariant = UnvalidatedTinyAsciiStr<8>;

#[icu_provider::data_struct(RegionDisplayNamesV1Marker = "displaynames/regions@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// RegionDisplayNames provides mapping between a region code and locale display name.
pub struct RegionDisplayNamesV1<'data> {
    /// Mapping for region to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedRegion, str>,
    /// Mapping for region to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedRegion, str>,
}

#[icu_provider::data_struct(LanguageDisplayNamesV1Marker = "displaynames/languages@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// LanguageDisplayNames provides mapping between languages and display names.
pub struct LanguageDisplayNamesV1<'data> {
    /// Mapping for language to display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to short display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to long display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to menu variant display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedLanguage, str>,
}

#[icu_provider::data_struct(ScriptDisplayNamesV1Marker = "displaynames/scripts@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// ScriptDisplayNames provides mapping between a script code and it's display name.
pub struct ScriptDisplayNamesV1<'data> {
    /// Mapping for script to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedScript, str>,
    /// Mapping for script to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedScript, str>,
}

#[icu_provider::data_struct(LocaleDisplayNamesV1Marker = "displaynames/locales@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// LocaleDisplayNames provides mapping between locales and display names.
pub struct LocaleDisplayNamesV1<'data> {
    /// Mapping for locale to display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to short display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to long display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to menu variant display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedLocale, str>,
}

#[icu_provider::data_struct(VariantDisplayNamesV1Marker = "displaynames/variants@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// VariantDisplayNames provides the user-translated names for the variant-code values.
pub struct VariantDisplayNamesV1<'data> {
    /// Mapping for Variant to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedVariant, str>,
}
