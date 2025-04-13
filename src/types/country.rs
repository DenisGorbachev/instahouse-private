use strum::EnumIter;
use strum::{Display, VariantArray};
#[allow(dead_code)]
pub use Country::*;

#[derive(Display, EnumIter, VariantArray, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Country {
    Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bhutan,
    Bolivia,
    BosniaAndHerzegovina,
    Botswana,
    Brazil,
    Brunei,
    Bulgaria,
    BurkinaFaso,
    Burundi,
    CaboVerde,
    Cambodia,
    Cameroon,
    Canada,
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    CongoBrazzaville,
    CongoKinshasa,
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    Czechia,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    EastTimor,
    Ecuador,
    Egypt,
    ElSalvador,
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Eswatini,
    Ethiopia,
    Fiji,
    Finland,
    France,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    Israel,
    Italy,
    IvoryCoast,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    KoreaNorth,
    KoreaSouth,
    Kosovo,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    MarshallIslands,
    Mauritania,
    Mauritius,
    Mexico,
    Micronesia,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    NorthMacedonia,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Palestine,
    Panama,
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Qatar,
    Romania,
    Russia,
    Rwanda,
    SaintKittsAndNevis,
    SaintLucia,
    SaintVincentAndTheGrenadines,
    Samoa,
    SanMarino,
    SaoTomeAndPrincipe,
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    SolomonIslands,
    Somalia,
    SouthAfrica,
    SouthSudan,
    Spain,
    SriLanka,
    Sudan,
    Suriname,
    Sweden,
    Switzerland,
    Syria,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    Togo,
    Tonga,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Uganda,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStates,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    Vatican,
    Venezuela,
    Vietnam,
    Yemen,
    Zambia,
    Zimbabwe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_enum_has_expected_number_of_variants() {
        assert_eq!(Country::VARIANTS.len(), 197);
    }
}
