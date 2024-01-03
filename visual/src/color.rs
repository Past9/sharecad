use auto_ops::impl_op_ex_commutative;
use image::{DynamicImage, GenericImage};
use space::{vec3, Vec3};

#[derive(Debug, Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Rgba {
    pub const TRANSPARENT: Self = rgba(0.0, 0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn clamped(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// Create a small texture image filled with this color.
    pub fn create_image(&self) -> image::DynamicImage {
        let mut image = DynamicImage::new_rgb8(2, 2);
        image.put_pixel(0, 0, image::Rgba::from(self.as_u8s()));
        image.put_pixel(0, 1, image::Rgba::from(self.as_u8s()));
        image.put_pixel(1, 0, image::Rgba::from(self.as_u8s()));
        image.put_pixel(1, 1, image::Rgba::from(self.as_u8s()));
        image
    }

    pub fn as_u8s(&self) -> [u8; 4] {
        [
            Self::f32_to_u8(self.r),
            Self::f32_to_u8(self.g),
            Self::f32_to_u8(self.b),
            Self::f32_to_u8(self.a),
        ]
    }

    pub fn as_f32s(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    fn f32_to_u8(val: f32) -> u8 {
        (val * 255.0).round() as u8
    }
}
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
    Rgba::new(r, g, b, a)
}

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Rgb {
    pub const AIR_FORCE_BLUE_RAF: Self = rgb(0.365, 0.541, 0.659);
    pub const AIR_FORCE_BLUE_USAF: Self = rgb(0.000, 0.188, 0.561);
    pub const AIR_SUPERIORITY_BLUE: Self = rgb(0.447, 0.627, 0.757);
    pub const ALABAMA_CRIMSON: Self = rgb(0.639, 0.149, 0.220);
    pub const ALICE_BLUE: Self = rgb(0.941, 0.973, 1.000);
    pub const ALIZARIN_CRIMSON: Self = rgb(0.890, 0.149, 0.212);
    pub const ALLOY_ORANGE: Self = rgb(0.769, 0.384, 0.063);
    pub const ALMOND: Self = rgb(0.937, 0.871, 0.804);
    pub const AMARANTH: Self = rgb(0.898, 0.169, 0.314);
    pub const AMBER: Self = rgb(1.000, 0.749, 0.000);
    pub const AMBER_SAE_ECE: Self = rgb(1.000, 0.494, 0.000);
    pub const AMERICAN_ROSE: Self = rgb(1.000, 0.012, 0.243);
    pub const AMETHYST: Self = rgb(0.600, 0.400, 0.800);
    pub const ANDROID_GREEN: Self = rgb(0.643, 0.776, 0.224);
    pub const ANTI_FLASH_WHITE: Self = rgb(0.949, 0.953, 0.957);
    pub const ANTIQUE_BRASS: Self = rgb(0.804, 0.584, 0.459);
    pub const ANTIQUE_FUCHSIA: Self = rgb(0.569, 0.361, 0.514);
    pub const ANTIQUE_RUBY: Self = rgb(0.518, 0.106, 0.176);
    pub const ANTIQUE_WHITE: Self = rgb(0.980, 0.922, 0.843);
    pub const AO_ENGLISH: Self = rgb(0.000, 0.502, 0.000);
    pub const APPLE_GREEN: Self = rgb(0.553, 0.714, 0.000);
    pub const APRICOT: Self = rgb(0.984, 0.808, 0.694);
    pub const AQUA: Self = rgb(0.000, 1.000, 1.000);
    pub const AQUAMARINE: Self = rgb(0.498, 1.000, 0.831);
    pub const ARMY_GREEN: Self = rgb(0.294, 0.325, 0.125);
    pub const ARSENIC: Self = rgb(0.231, 0.267, 0.294);
    pub const ARYLIDE_YELLOW: Self = rgb(0.914, 0.839, 0.420);
    pub const ASH_GREY: Self = rgb(0.698, 0.745, 0.710);
    pub const ASPARAGUS: Self = rgb(0.529, 0.663, 0.420);
    pub const ATOMIC_TANGERINE: Self = rgb(1.000, 0.600, 0.400);
    pub const AUBURN: Self = rgb(0.647, 0.165, 0.165);
    pub const AUREOLIN: Self = rgb(0.992, 0.933, 0.000);
    pub const AUROMETALSAURUS: Self = rgb(0.431, 0.498, 0.502);
    pub const AVOCADO: Self = rgb(0.337, 0.510, 0.012);
    pub const AZURE: Self = rgb(0.000, 0.498, 1.000);
    pub const AZURE_MIST_WEB: Self = rgb(0.941, 1.000, 1.000);
    pub const BABY_BLUE: Self = rgb(0.537, 0.812, 0.941);
    pub const BABY_BLUE_EYES: Self = rgb(0.631, 0.792, 0.945);
    pub const BABY_PINK: Self = rgb(0.957, 0.761, 0.761);
    pub const BALL_BLUE: Self = rgb(0.129, 0.671, 0.804);
    pub const BANANA_MANIA: Self = rgb(0.980, 0.906, 0.710);
    pub const BANANA_YELLOW: Self = rgb(1.000, 0.882, 0.208);
    pub const BARN_RED: Self = rgb(0.486, 0.039, 0.008);
    pub const BATTLESHIP_GREY: Self = rgb(0.518, 0.518, 0.510);
    pub const BAZAAR: Self = rgb(0.596, 0.467, 0.482);
    pub const BEAU_BLUE: Self = rgb(0.737, 0.831, 0.902);
    pub const BEAVER: Self = rgb(0.624, 0.506, 0.439);
    pub const BEIGE: Self = rgb(0.961, 0.961, 0.863);
    pub const BIG_DIP_O_RUBY: Self = rgb(0.612, 0.145, 0.259);
    pub const BISQUE: Self = rgb(1.000, 0.894, 0.769);
    pub const BISTRE: Self = rgb(0.239, 0.169, 0.122);
    pub const BITTERSWEET: Self = rgb(0.996, 0.435, 0.369);
    pub const BITTERSWEET_SHIMMER: Self = rgb(0.749, 0.310, 0.318);
    pub const BLACK: Self = rgb(0.000, 0.000, 0.000);
    pub const BLACK_BEAN: Self = rgb(0.239, 0.047, 0.008);
    pub const BLACK_LEATHER_JACKET: Self = rgb(0.145, 0.208, 0.161);
    pub const BLACK_OLIVE: Self = rgb(0.231, 0.235, 0.212);
    pub const BLANCHED_ALMOND: Self = rgb(1.000, 0.922, 0.804);
    pub const BLAST_OFF_BRONZE: Self = rgb(0.647, 0.443, 0.392);
    pub const BLEU_DE_FRANCE: Self = rgb(0.192, 0.549, 0.906);
    pub const BLIZZARD_BLUE: Self = rgb(0.675, 0.898, 0.933);
    pub const BLOND: Self = rgb(0.980, 0.941, 0.745);
    pub const BLUE: Self = rgb(0.000, 0.000, 1.000);
    pub const BLUE_BELL: Self = rgb(0.635, 0.635, 0.816);
    pub const BLUE_CRAYOLA: Self = rgb(0.122, 0.459, 0.996);
    pub const BLUE_GRAY: Self = rgb(0.400, 0.600, 0.800);
    pub const BLUE_GREEN: Self = rgb(0.051, 0.596, 0.729);
    pub const BLUE_MUNSELL: Self = rgb(0.000, 0.576, 0.686);
    pub const BLUE_NCS: Self = rgb(0.000, 0.529, 0.741);
    pub const BLUE_PIGMENT: Self = rgb(0.200, 0.200, 0.600);
    pub const BLUE_RYB: Self = rgb(0.008, 0.278, 0.996);
    pub const BLUE_SAPPHIRE: Self = rgb(0.071, 0.380, 0.502);
    pub const BLUE_VIOLET: Self = rgb(0.541, 0.169, 0.886);
    pub const BLUSH: Self = rgb(0.871, 0.365, 0.514);
    pub const BOLE: Self = rgb(0.475, 0.267, 0.231);
    pub const BONDI_BLUE: Self = rgb(0.000, 0.584, 0.714);
    pub const BONE: Self = rgb(0.890, 0.855, 0.788);
    pub const BOSTON_UNIVERSITY_RED: Self = rgb(0.800, 0.000, 0.000);
    pub const BOTTLE_GREEN: Self = rgb(0.000, 0.416, 0.306);
    pub const BOYSENBERRY: Self = rgb(0.529, 0.196, 0.376);
    pub const BRANDEIS_BLUE: Self = rgb(0.000, 0.439, 1.000);
    pub const BRASS: Self = rgb(0.710, 0.651, 0.259);
    pub const BRICK_RED: Self = rgb(0.796, 0.255, 0.329);
    pub const BRIGHT_CERULEAN: Self = rgb(0.114, 0.675, 0.839);
    pub const BRIGHT_GREEN: Self = rgb(0.400, 1.000, 0.000);
    pub const BRIGHT_LAVENDER: Self = rgb(0.749, 0.580, 0.894);
    pub const BRIGHT_MAROON: Self = rgb(0.765, 0.129, 0.282);
    pub const BRIGHT_PINK: Self = rgb(1.000, 0.000, 0.498);
    pub const BRIGHT_TURQUOISE: Self = rgb(0.031, 0.910, 0.871);
    pub const BRIGHT_UBE: Self = rgb(0.820, 0.624, 0.910);
    pub const BRILLIANT_LAVENDER: Self = rgb(0.957, 0.733, 1.000);
    pub const BRILLIANT_ROSE: Self = rgb(1.000, 0.333, 0.639);
    pub const BRINK_PINK: Self = rgb(0.984, 0.376, 0.498);
    pub const BRITISH_RACING_GREEN: Self = rgb(0.000, 0.259, 0.145);
    pub const BRONZE: Self = rgb(0.804, 0.498, 0.196);
    pub const BROWN_TRADITIONAL: Self = rgb(0.588, 0.294, 0.000);
    pub const BROWN_WEB: Self = rgb(0.647, 0.165, 0.165);
    pub const BUBBLE_GUM: Self = rgb(1.000, 0.757, 0.800);
    pub const BUBBLES: Self = rgb(0.906, 0.996, 1.000);
    pub const BUFF: Self = rgb(0.941, 0.863, 0.510);
    pub const BULGARIAN_ROSE: Self = rgb(0.282, 0.024, 0.027);
    pub const BURGUNDY: Self = rgb(0.502, 0.000, 0.125);
    pub const BURLYWOOD: Self = rgb(0.871, 0.722, 0.529);
    pub const BURNT_ORANGE: Self = rgb(0.800, 0.333, 0.000);
    pub const BURNT_SIENNA: Self = rgb(0.914, 0.455, 0.318);
    pub const BURNT_UMBER: Self = rgb(0.541, 0.200, 0.141);
    pub const BYZANTINE: Self = rgb(0.741, 0.200, 0.643);
    pub const BYZANTIUM: Self = rgb(0.439, 0.161, 0.388);
    pub const CADET: Self = rgb(0.325, 0.408, 0.447);
    pub const CADET_BLUE: Self = rgb(0.373, 0.620, 0.627);
    pub const CADET_GREY: Self = rgb(0.569, 0.639, 0.690);
    pub const CADMIUM_GREEN: Self = rgb(0.000, 0.420, 0.235);
    pub const CADMIUM_ORANGE: Self = rgb(0.929, 0.529, 0.176);
    pub const CADMIUM_RED: Self = rgb(0.890, 0.000, 0.133);
    pub const CADMIUM_YELLOW: Self = rgb(1.000, 0.965, 0.000);
    pub const CAF_AU_LAIT: Self = rgb(0.651, 0.482, 0.357);
    pub const CAF_NOIR: Self = rgb(0.294, 0.212, 0.129);
    pub const CAL_POLY_GREEN: Self = rgb(0.118, 0.302, 0.169);
    pub const CAMBRIDGE_BLUE: Self = rgb(0.639, 0.757, 0.678);
    pub const CAMEL: Self = rgb(0.757, 0.604, 0.420);
    pub const CAMEO_PINK: Self = rgb(0.937, 0.733, 0.800);
    pub const CAMOUFLAGE_GREEN: Self = rgb(0.471, 0.525, 0.420);
    pub const CANARY_YELLOW: Self = rgb(1.000, 0.937, 0.000);
    pub const CANDY_APPLE_RED: Self = rgb(1.000, 0.031, 0.000);
    pub const CANDY_PINK: Self = rgb(0.894, 0.443, 0.478);
    pub const CAPRI: Self = rgb(0.000, 0.749, 1.000);
    pub const CAPUT_MORTUUM: Self = rgb(0.349, 0.153, 0.125);
    pub const CARDINAL: Self = rgb(0.769, 0.118, 0.227);
    pub const CARIBBEAN_GREEN: Self = rgb(0.000, 0.800, 0.600);
    pub const CARMINE: Self = rgb(0.588, 0.000, 0.094);
    pub const CARMINE_M_P: Self = rgb(0.843, 0.000, 0.251);
    pub const CARMINE_PINK: Self = rgb(0.922, 0.298, 0.259);
    pub const CARMINE_RED: Self = rgb(1.000, 0.000, 0.220);
    pub const CARNATION_PINK: Self = rgb(1.000, 0.651, 0.788);
    pub const CARNELIAN: Self = rgb(0.702, 0.106, 0.106);
    pub const CAROLINA_BLUE: Self = rgb(0.600, 0.729, 0.867);
    pub const CARROT_ORANGE: Self = rgb(0.929, 0.569, 0.129);
    pub const CATALINA_BLUE: Self = rgb(0.024, 0.165, 0.471);
    pub const CEIL: Self = rgb(0.573, 0.631, 0.812);
    pub const CELADON: Self = rgb(0.675, 0.882, 0.686);
    pub const CELADON_BLUE: Self = rgb(0.000, 0.482, 0.655);
    pub const CELADON_GREEN: Self = rgb(0.184, 0.518, 0.486);
    pub const CELESTE_COLOUR: Self = rgb(0.698, 1.000, 1.000);
    pub const CELESTIAL_BLUE: Self = rgb(0.286, 0.592, 0.816);
    pub const CERISE: Self = rgb(0.871, 0.192, 0.388);
    pub const CERISE_PINK: Self = rgb(0.925, 0.231, 0.514);
    pub const CERULEAN: Self = rgb(0.000, 0.482, 0.655);
    pub const CERULEAN_BLUE: Self = rgb(0.165, 0.322, 0.745);
    pub const CERULEAN_FROST: Self = rgb(0.427, 0.608, 0.765);
    pub const CG_BLUE: Self = rgb(0.000, 0.478, 0.647);
    pub const CG_RED: Self = rgb(0.878, 0.235, 0.192);
    pub const CHAMOISEE: Self = rgb(0.627, 0.471, 0.353);
    pub const CHAMPAGNE: Self = rgb(0.980, 0.839, 0.647);
    pub const CHARCOAL: Self = rgb(0.212, 0.271, 0.310);
    pub const CHARM_PINK: Self = rgb(0.902, 0.561, 0.675);
    pub const CHARTREUSE_TRADITIONAL: Self = rgb(0.875, 1.000, 0.000);
    pub const CHARTREUSE_WEB: Self = rgb(0.498, 1.000, 0.000);
    pub const CHERRY: Self = rgb(0.871, 0.192, 0.388);
    pub const CHERRY_BLOSSOM_PINK: Self = rgb(1.000, 0.718, 0.773);
    pub const CHESTNUT: Self = rgb(0.804, 0.361, 0.361);
    pub const CHINA_PINK: Self = rgb(0.871, 0.435, 0.631);
    pub const CHINA_ROSE: Self = rgb(0.659, 0.318, 0.431);
    pub const CHINESE_RED: Self = rgb(0.667, 0.220, 0.118);
    pub const CHOCOLATE_TRADITIONAL: Self = rgb(0.482, 0.247, 0.000);
    pub const CHOCOLATE_WEB: Self = rgb(0.824, 0.412, 0.118);
    pub const CHROME_YELLOW: Self = rgb(1.000, 0.655, 0.000);
    pub const CINEREOUS: Self = rgb(0.596, 0.506, 0.482);
    pub const CINNABAR: Self = rgb(0.890, 0.259, 0.204);
    pub const CINNAMON: Self = rgb(0.824, 0.412, 0.118);
    pub const CITRINE: Self = rgb(0.894, 0.816, 0.039);
    pub const CLASSIC_ROSE: Self = rgb(0.984, 0.800, 0.906);
    pub const COBALT: Self = rgb(0.000, 0.278, 0.671);
    pub const COCOA_BROWN: Self = rgb(0.824, 0.412, 0.118);
    pub const COFFEE: Self = rgb(0.435, 0.306, 0.216);
    pub const COLUMBIA_BLUE: Self = rgb(0.608, 0.867, 1.000);
    pub const CONGO_PINK: Self = rgb(0.973, 0.514, 0.475);
    pub const COOL_BLACK: Self = rgb(0.000, 0.180, 0.388);
    pub const COOL_GREY: Self = rgb(0.549, 0.573, 0.675);
    pub const COPPER: Self = rgb(0.722, 0.451, 0.200);
    pub const COPPER_CRAYOLA: Self = rgb(0.855, 0.541, 0.404);
    pub const COPPER_PENNY: Self = rgb(0.678, 0.435, 0.412);
    pub const COPPER_RED: Self = rgb(0.796, 0.427, 0.318);
    pub const COPPER_ROSE: Self = rgb(0.600, 0.400, 0.400);
    pub const COQUELICOT: Self = rgb(1.000, 0.220, 0.000);
    pub const CORAL: Self = rgb(1.000, 0.498, 0.314);
    pub const CORAL_PINK: Self = rgb(0.973, 0.514, 0.475);
    pub const CORAL_RED: Self = rgb(1.000, 0.251, 0.251);
    pub const CORDOVAN: Self = rgb(0.537, 0.247, 0.271);
    pub const CORN: Self = rgb(0.984, 0.925, 0.365);
    pub const CORNELL_RED: Self = rgb(0.702, 0.106, 0.106);
    pub const CORNFLOWER_BLUE: Self = rgb(0.392, 0.584, 0.929);
    pub const CORNSILK: Self = rgb(1.000, 0.973, 0.863);
    pub const COSMIC_LATTE: Self = rgb(1.000, 0.973, 0.906);
    pub const COTTON_CANDY: Self = rgb(1.000, 0.737, 0.851);
    pub const CREAM: Self = rgb(1.000, 0.992, 0.816);
    pub const CRIMSON: Self = rgb(0.863, 0.078, 0.235);
    pub const CRIMSON_GLORY: Self = rgb(0.745, 0.000, 0.196);
    pub const CYAN: Self = rgb(0.000, 1.000, 1.000);
    pub const CYAN_PROCESS: Self = rgb(0.000, 0.718, 0.922);
    pub const DAFFODIL: Self = rgb(1.000, 1.000, 0.192);
    pub const DANDELION: Self = rgb(0.941, 0.882, 0.188);
    pub const DARK_BLUE: Self = rgb(0.000, 0.000, 0.545);
    pub const DARK_BROWN: Self = rgb(0.396, 0.263, 0.129);
    pub const DARK_BYZANTIUM: Self = rgb(0.365, 0.224, 0.329);
    pub const DARK_CANDY_APPLE_RED: Self = rgb(0.643, 0.000, 0.000);
    pub const DARK_CERULEAN: Self = rgb(0.031, 0.271, 0.494);
    pub const DARK_CHESTNUT: Self = rgb(0.596, 0.412, 0.376);
    pub const DARK_CORAL: Self = rgb(0.804, 0.357, 0.271);
    pub const DARK_CYAN: Self = rgb(0.000, 0.545, 0.545);
    pub const DARK_ELECTRIC_BLUE: Self = rgb(0.325, 0.408, 0.471);
    pub const DARK_GOLDENROD: Self = rgb(0.722, 0.525, 0.043);
    pub const DARK_GRAY: Self = rgb(0.663, 0.663, 0.663);
    pub const DARK_GREEN: Self = rgb(0.004, 0.196, 0.125);
    pub const DARK_IMPERIAL_BLUE: Self = rgb(0.000, 0.255, 0.416);
    pub const DARK_JUNGLE_GREEN: Self = rgb(0.102, 0.141, 0.129);
    pub const DARK_KHAKI: Self = rgb(0.741, 0.718, 0.420);
    pub const DARK_LAVA: Self = rgb(0.282, 0.235, 0.196);
    pub const DARK_LAVENDER: Self = rgb(0.451, 0.310, 0.588);
    pub const DARK_MAGENTA: Self = rgb(0.545, 0.000, 0.545);
    pub const DARK_MIDNIGHT_BLUE: Self = rgb(0.000, 0.200, 0.400);
    pub const DARK_OLIVE_GREEN: Self = rgb(0.333, 0.420, 0.184);
    pub const DARK_ORANGE: Self = rgb(1.000, 0.549, 0.000);
    pub const DARK_ORCHID: Self = rgb(0.600, 0.196, 0.800);
    pub const DARK_PASTEL_BLUE: Self = rgb(0.467, 0.620, 0.796);
    pub const DARK_PASTEL_GREEN: Self = rgb(0.012, 0.753, 0.235);
    pub const DARK_PASTEL_PURPLE: Self = rgb(0.588, 0.435, 0.839);
    pub const DARK_PASTEL_RED: Self = rgb(0.761, 0.231, 0.133);
    pub const DARK_PINK: Self = rgb(0.906, 0.329, 0.502);
    pub const DARK_POWDER_BLUE: Self = rgb(0.000, 0.200, 0.600);
    pub const DARK_RASPBERRY: Self = rgb(0.529, 0.149, 0.341);
    pub const DARK_RED: Self = rgb(0.545, 0.000, 0.000);
    pub const DARK_SALMON: Self = rgb(0.914, 0.588, 0.478);
    pub const DARK_SCARLET: Self = rgb(0.337, 0.012, 0.098);
    pub const DARK_SEA_GREEN: Self = rgb(0.561, 0.737, 0.561);
    pub const DARK_SIENNA: Self = rgb(0.235, 0.078, 0.078);
    pub const DARK_SLATE_BLUE: Self = rgb(0.282, 0.239, 0.545);
    pub const DARK_SLATE_GRAY: Self = rgb(0.184, 0.310, 0.310);
    pub const DARK_SPRING_GREEN: Self = rgb(0.090, 0.447, 0.271);
    pub const DARK_TAN: Self = rgb(0.569, 0.506, 0.318);
    pub const DARK_TANGERINE: Self = rgb(1.000, 0.659, 0.071);
    pub const DARK_TAUPE: Self = rgb(0.282, 0.235, 0.196);
    pub const DARK_TERRA_COTTA: Self = rgb(0.800, 0.306, 0.361);
    pub const DARK_TURQUOISE: Self = rgb(0.000, 0.808, 0.820);
    pub const DARK_VIOLET: Self = rgb(0.580, 0.000, 0.827);
    pub const DARK_YELLOW: Self = rgb(0.608, 0.529, 0.047);
    pub const DARTMOUTH_GREEN: Self = rgb(0.000, 0.439, 0.235);
    pub const DAVY_S_GREY: Self = rgb(0.333, 0.333, 0.333);
    pub const DEBIAN_RED: Self = rgb(0.843, 0.039, 0.325);
    pub const DEEP_CARMINE: Self = rgb(0.663, 0.125, 0.243);
    pub const DEEP_CARMINE_PINK: Self = rgb(0.937, 0.188, 0.220);
    pub const DEEP_CARROT_ORANGE: Self = rgb(0.914, 0.412, 0.173);
    pub const DEEP_CERISE: Self = rgb(0.855, 0.196, 0.529);
    pub const DEEP_CHAMPAGNE: Self = rgb(0.980, 0.839, 0.647);
    pub const DEEP_CHESTNUT: Self = rgb(0.725, 0.306, 0.282);
    pub const DEEP_COFFEE: Self = rgb(0.439, 0.259, 0.255);
    pub const DEEP_FUCHSIA: Self = rgb(0.757, 0.329, 0.757);
    pub const DEEP_JUNGLE_GREEN: Self = rgb(0.000, 0.294, 0.286);
    pub const DEEP_LILAC: Self = rgb(0.600, 0.333, 0.733);
    pub const DEEP_MAGENTA: Self = rgb(0.800, 0.000, 0.800);
    pub const DEEP_PEACH: Self = rgb(1.000, 0.796, 0.643);
    pub const DEEP_PINK: Self = rgb(1.000, 0.078, 0.576);
    pub const DEEP_RUBY: Self = rgb(0.518, 0.247, 0.357);
    pub const DEEP_SAFFRON: Self = rgb(1.000, 0.600, 0.200);
    pub const DEEP_SKY_BLUE: Self = rgb(0.000, 0.749, 1.000);
    pub const DEEP_TUSCAN_RED: Self = rgb(0.400, 0.259, 0.302);
    pub const DENIM: Self = rgb(0.082, 0.376, 0.741);
    pub const DESERT: Self = rgb(0.757, 0.604, 0.420);
    pub const DESERT_SAND: Self = rgb(0.929, 0.788, 0.686);
    pub const DIM_GRAY: Self = rgb(0.412, 0.412, 0.412);
    pub const DODGER_BLUE: Self = rgb(0.118, 0.565, 1.000);
    pub const DOGWOOD_ROSE: Self = rgb(0.843, 0.094, 0.408);
    pub const DOLLAR_BILL: Self = rgb(0.522, 0.733, 0.396);
    pub const DRAB: Self = rgb(0.588, 0.443, 0.090);
    pub const DUKE_BLUE: Self = rgb(0.000, 0.000, 0.612);
    pub const EARTH_YELLOW: Self = rgb(0.882, 0.663, 0.373);
    pub const EBONY: Self = rgb(0.333, 0.365, 0.314);
    pub const ECRU: Self = rgb(0.761, 0.698, 0.502);
    pub const EGGPLANT: Self = rgb(0.380, 0.251, 0.318);
    pub const EGGSHELL: Self = rgb(0.941, 0.918, 0.839);
    pub const EGYPTIAN_BLUE: Self = rgb(0.063, 0.204, 0.651);
    pub const ELECTRIC_BLUE: Self = rgb(0.490, 0.976, 1.000);
    pub const ELECTRIC_CRIMSON: Self = rgb(1.000, 0.000, 0.247);
    pub const ELECTRIC_CYAN: Self = rgb(0.000, 1.000, 1.000);
    pub const ELECTRIC_GREEN: Self = rgb(0.000, 1.000, 0.000);
    pub const ELECTRIC_INDIGO: Self = rgb(0.435, 0.000, 1.000);
    pub const ELECTRIC_LAVENDER: Self = rgb(0.957, 0.733, 1.000);
    pub const ELECTRIC_LIME: Self = rgb(0.800, 1.000, 0.000);
    pub const ELECTRIC_PURPLE: Self = rgb(0.749, 0.000, 1.000);
    pub const ELECTRIC_ULTRAMARINE: Self = rgb(0.247, 0.000, 1.000);
    pub const ELECTRIC_VIOLET: Self = rgb(0.561, 0.000, 1.000);
    pub const ELECTRIC_YELLOW: Self = rgb(1.000, 1.000, 0.000);
    pub const EMERALD: Self = rgb(0.314, 0.784, 0.471);
    pub const ENGLISH_LAVENDER: Self = rgb(0.706, 0.514, 0.584);
    pub const ETON_BLUE: Self = rgb(0.588, 0.784, 0.635);
    pub const FALLOW: Self = rgb(0.757, 0.604, 0.420);
    pub const FALU_RED: Self = rgb(0.502, 0.094, 0.094);
    pub const FANDANGO: Self = rgb(0.710, 0.200, 0.537);
    pub const FASHION_FUCHSIA: Self = rgb(0.957, 0.000, 0.631);
    pub const FAWN: Self = rgb(0.898, 0.667, 0.439);
    pub const FELDGRAU: Self = rgb(0.302, 0.365, 0.325);
    pub const FERN_GREEN: Self = rgb(0.310, 0.475, 0.259);
    pub const FERRARI_RED: Self = rgb(1.000, 0.157, 0.000);
    pub const FIELD_DRAB: Self = rgb(0.424, 0.329, 0.118);
    pub const FIRE_ENGINE_RED: Self = rgb(0.808, 0.125, 0.161);
    pub const FIREBRICK: Self = rgb(0.698, 0.133, 0.133);
    pub const FLAME: Self = rgb(0.886, 0.345, 0.133);
    pub const FLAMINGO_PINK: Self = rgb(0.988, 0.557, 0.675);
    pub const FLAVESCENT: Self = rgb(0.969, 0.914, 0.557);
    pub const FLAX: Self = rgb(0.933, 0.863, 0.510);
    pub const FLORAL_WHITE: Self = rgb(1.000, 0.980, 0.941);
    pub const FLUORESCENT_ORANGE: Self = rgb(1.000, 0.749, 0.000);
    pub const FLUORESCENT_PINK: Self = rgb(1.000, 0.078, 0.576);
    pub const FLUORESCENT_YELLOW: Self = rgb(0.800, 1.000, 0.000);
    pub const FOLLY: Self = rgb(1.000, 0.000, 0.310);
    pub const FOREST_GREEN_TRADITIONAL: Self = rgb(0.004, 0.267, 0.129);
    pub const FOREST_GREEN_WEB: Self = rgb(0.133, 0.545, 0.133);
    pub const FRENCH_BEIGE: Self = rgb(0.651, 0.482, 0.357);
    pub const FRENCH_BLUE: Self = rgb(0.000, 0.447, 0.733);
    pub const FRENCH_LILAC: Self = rgb(0.525, 0.376, 0.557);
    pub const FRENCH_LIME: Self = rgb(0.800, 1.000, 0.000);
    pub const FRENCH_RASPBERRY: Self = rgb(0.780, 0.173, 0.282);
    pub const FRENCH_ROSE: Self = rgb(0.965, 0.290, 0.541);
    pub const FUCHSIA: Self = rgb(1.000, 0.000, 1.000);
    pub const FUCHSIA_CRAYOLA: Self = rgb(0.757, 0.329, 0.757);
    pub const FUCHSIA_PINK: Self = rgb(1.000, 0.467, 1.000);
    pub const FUCHSIA_ROSE: Self = rgb(0.780, 0.263, 0.459);
    pub const FULVOUS: Self = rgb(0.894, 0.518, 0.000);
    pub const FUZZY_WUZZY: Self = rgb(0.800, 0.400, 0.400);
    pub const GAINSBORO: Self = rgb(0.863, 0.863, 0.863);
    pub const GAMBOGE: Self = rgb(0.894, 0.608, 0.059);
    pub const GHOST_WHITE: Self = rgb(0.973, 0.973, 1.000);
    pub const GINGER: Self = rgb(0.690, 0.396, 0.000);
    pub const GLAUCOUS: Self = rgb(0.376, 0.510, 0.714);
    pub const GLITTER: Self = rgb(0.902, 0.910, 0.980);
    pub const GOLD_METALLIC: Self = rgb(0.831, 0.686, 0.216);
    pub const GOLD_WEB_GOLDEN: Self = rgb(1.000, 0.843, 0.000);
    pub const GOLDEN_BROWN: Self = rgb(0.600, 0.396, 0.082);
    pub const GOLDEN_POPPY: Self = rgb(0.988, 0.761, 0.000);
    pub const GOLDEN_YELLOW: Self = rgb(1.000, 0.875, 0.000);
    pub const GOLDENROD: Self = rgb(0.855, 0.647, 0.125);
    pub const GRANNY_SMITH_APPLE: Self = rgb(0.659, 0.894, 0.627);
    pub const GRAY: Self = rgb(0.502, 0.502, 0.502);
    pub const GRAY_ASPARAGUS: Self = rgb(0.275, 0.349, 0.271);
    pub const GRAY_HTML_CSS_GRAY: Self = rgb(0.502, 0.502, 0.502);
    pub const GRAY_X11_GRAY: Self = rgb(0.745, 0.745, 0.745);
    pub const GREEN_COLOR_WHEEL_X11_GREEN: Self = rgb(0.000, 1.000, 0.000);
    pub const GREEN_CRAYOLA: Self = rgb(0.110, 0.675, 0.471);
    pub const GREEN_HTML_CSS_GREEN: Self = rgb(0.000, 0.502, 0.000);
    pub const GREEN_MUNSELL: Self = rgb(0.000, 0.659, 0.467);
    pub const GREEN_NCS: Self = rgb(0.000, 0.624, 0.420);
    pub const GREEN_PIGMENT: Self = rgb(0.000, 0.647, 0.314);
    pub const GREEN_RYB: Self = rgb(0.400, 0.690, 0.196);
    pub const GREEN_YELLOW: Self = rgb(0.678, 1.000, 0.184);
    pub const GRULLO: Self = rgb(0.663, 0.604, 0.525);
    pub const GUPPIE_GREEN: Self = rgb(0.000, 1.000, 0.498);
    pub const HALAY_BE: Self = rgb(0.400, 0.220, 0.329);
    pub const HAN_BLUE: Self = rgb(0.267, 0.424, 0.812);
    pub const HAN_PURPLE: Self = rgb(0.322, 0.094, 0.980);
    pub const HANSA_YELLOW: Self = rgb(0.914, 0.839, 0.420);
    pub const HARLEQUIN: Self = rgb(0.247, 1.000, 0.000);
    pub const HARVARD_CRIMSON: Self = rgb(0.788, 0.000, 0.086);
    pub const HARVEST_GOLD: Self = rgb(0.855, 0.569, 0.000);
    pub const HEART_GOLD: Self = rgb(0.502, 0.502, 0.000);
    pub const HELIOTROPE: Self = rgb(0.875, 0.451, 1.000);
    pub const HOLLYWOOD_CERISE: Self = rgb(0.957, 0.000, 0.631);
    pub const HONEYDEW: Self = rgb(0.941, 1.000, 0.941);
    pub const HONOLULU_BLUE: Self = rgb(0.000, 0.498, 0.749);
    pub const HOOKER_S_GREEN: Self = rgb(0.286, 0.475, 0.420);
    pub const HOT_MAGENTA: Self = rgb(1.000, 0.114, 0.808);
    pub const HOT_PINK: Self = rgb(1.000, 0.412, 0.706);
    pub const HUNTER_GREEN: Self = rgb(0.208, 0.369, 0.231);
    pub const ICEBERG: Self = rgb(0.443, 0.651, 0.824);
    pub const ICTERINE: Self = rgb(0.988, 0.969, 0.369);
    pub const IMPERIAL_BLUE: Self = rgb(0.000, 0.137, 0.584);
    pub const INCHWORM: Self = rgb(0.698, 0.925, 0.365);
    pub const INDIA_GREEN: Self = rgb(0.075, 0.533, 0.031);
    pub const INDIAN_RED: Self = rgb(0.804, 0.361, 0.361);
    pub const INDIAN_YELLOW: Self = rgb(0.890, 0.659, 0.341);
    pub const INDIGO: Self = rgb(0.435, 0.000, 1.000);
    pub const INDIGO_DYE: Self = rgb(0.000, 0.255, 0.416);
    pub const INDIGO_WEB: Self = rgb(0.294, 0.000, 0.510);
    pub const INTERNATIONAL_KLEIN_BLUE: Self = rgb(0.000, 0.184, 0.655);
    pub const INTERNATIONAL_ORANGE_AEROSPACE: Self = rgb(1.000, 0.310, 0.000);
    pub const INTERNATIONAL_ORANGE_ENGINEERING: Self = rgb(0.729, 0.086, 0.047);
    pub const INTERNATIONAL_ORANGE_GOLDEN_GATE_BRIDGE: Self = rgb(0.753, 0.212, 0.173);
    pub const IRIS: Self = rgb(0.353, 0.310, 0.812);
    pub const ISABELLINE: Self = rgb(0.957, 0.941, 0.925);
    pub const ISLAMIC_GREEN: Self = rgb(0.000, 0.565, 0.000);
    pub const IVORY: Self = rgb(1.000, 1.000, 0.941);
    pub const JADE: Self = rgb(0.000, 0.659, 0.420);
    pub const JASMINE: Self = rgb(0.973, 0.871, 0.494);
    pub const JASPER: Self = rgb(0.843, 0.231, 0.243);
    pub const JAZZBERRY_JAM: Self = rgb(0.647, 0.043, 0.369);
    pub const JET: Self = rgb(0.204, 0.204, 0.204);
    pub const JONQUIL: Self = rgb(0.980, 0.855, 0.369);
    pub const JUNE_BUD: Self = rgb(0.741, 0.855, 0.341);
    pub const JUNGLE_GREEN: Self = rgb(0.161, 0.671, 0.529);
    pub const KELLY_GREEN: Self = rgb(0.298, 0.733, 0.090);
    pub const KENYAN_COPPER: Self = rgb(0.486, 0.110, 0.020);
    pub const KHAKI_HTML_CSS_KHAKI: Self = rgb(0.765, 0.690, 0.569);
    pub const KHAKI_X11_LIGHT_KHAKI: Self = rgb(0.941, 0.902, 0.549);
    pub const KU_CRIMSON: Self = rgb(0.910, 0.000, 0.051);
    pub const LA_SALLE_GREEN: Self = rgb(0.031, 0.471, 0.188);
    pub const LANGUID_LAVENDER: Self = rgb(0.839, 0.792, 0.867);
    pub const LAPIS_LAZULI: Self = rgb(0.149, 0.380, 0.612);
    pub const LASER_LEMON: Self = rgb(0.996, 0.996, 0.133);
    pub const LAUREL_GREEN: Self = rgb(0.663, 0.729, 0.616);
    pub const LAVA: Self = rgb(0.812, 0.063, 0.125);
    pub const LAVENDER_BLUE: Self = rgb(0.800, 0.800, 1.000);
    pub const LAVENDER_BLUSH: Self = rgb(1.000, 0.941, 0.961);
    pub const LAVENDER_FLORAL: Self = rgb(0.710, 0.494, 0.863);
    pub const LAVENDER_GRAY: Self = rgb(0.769, 0.765, 0.816);
    pub const LAVENDER_INDIGO: Self = rgb(0.580, 0.341, 0.922);
    pub const LAVENDER_MAGENTA: Self = rgb(0.933, 0.510, 0.933);
    pub const LAVENDER_MIST: Self = rgb(0.902, 0.902, 0.980);
    pub const LAVENDER_PINK: Self = rgb(0.984, 0.682, 0.824);
    pub const LAVENDER_PURPLE: Self = rgb(0.588, 0.482, 0.714);
    pub const LAVENDER_ROSE: Self = rgb(0.984, 0.627, 0.890);
    pub const LAVENDER_WEB: Self = rgb(0.902, 0.902, 0.980);
    pub const LAWN_GREEN: Self = rgb(0.486, 0.988, 0.000);
    pub const LEMON: Self = rgb(1.000, 0.969, 0.000);
    pub const LEMON_CHIFFON: Self = rgb(1.000, 0.980, 0.804);
    pub const LEMON_LIME: Self = rgb(0.890, 1.000, 0.000);
    pub const LICORICE: Self = rgb(0.102, 0.067, 0.063);
    pub const LIGHT_APRICOT: Self = rgb(0.992, 0.835, 0.694);
    pub const LIGHT_BLUE: Self = rgb(0.678, 0.847, 0.902);
    pub const LIGHT_BROWN: Self = rgb(0.710, 0.396, 0.114);
    pub const LIGHT_CARMINE_PINK: Self = rgb(0.902, 0.404, 0.443);
    pub const LIGHT_CORAL: Self = rgb(0.941, 0.502, 0.502);
    pub const LIGHT_CORNFLOWER_BLUE: Self = rgb(0.576, 0.800, 0.918);
    pub const LIGHT_CRIMSON: Self = rgb(0.961, 0.412, 0.569);
    pub const LIGHT_CYAN: Self = rgb(0.878, 1.000, 1.000);
    pub const LIGHT_FUCHSIA_PINK: Self = rgb(0.976, 0.518, 0.937);
    pub const LIGHT_GOLDENROD_YELLOW: Self = rgb(0.980, 0.980, 0.824);
    pub const LIGHT_GRAY: Self = rgb(0.827, 0.827, 0.827);
    pub const LIGHT_GREEN: Self = rgb(0.565, 0.933, 0.565);
    pub const LIGHT_KHAKI: Self = rgb(0.941, 0.902, 0.549);
    pub const LIGHT_PASTEL_PURPLE: Self = rgb(0.694, 0.612, 0.851);
    pub const LIGHT_PINK: Self = rgb(1.000, 0.714, 0.757);
    pub const LIGHT_RED_OCHRE: Self = rgb(0.914, 0.455, 0.318);
    pub const LIGHT_SALMON: Self = rgb(1.000, 0.627, 0.478);
    pub const LIGHT_SALMON_PINK: Self = rgb(1.000, 0.600, 0.600);
    pub const LIGHT_SEA_GREEN: Self = rgb(0.125, 0.698, 0.667);
    pub const LIGHT_SKY_BLUE: Self = rgb(0.529, 0.808, 0.980);
    pub const LIGHT_SLATE_GRAY: Self = rgb(0.467, 0.533, 0.600);
    pub const LIGHT_TAUPE: Self = rgb(0.702, 0.545, 0.427);
    pub const LIGHT_THULIAN_PINK: Self = rgb(0.902, 0.561, 0.675);
    pub const LIGHT_YELLOW: Self = rgb(1.000, 1.000, 0.878);
    pub const LILAC: Self = rgb(0.784, 0.635, 0.784);
    pub const LIME_COLOR_WHEEL: Self = rgb(0.749, 1.000, 0.000);
    pub const LIME_GREEN: Self = rgb(0.196, 0.804, 0.196);
    pub const LIME_WEB_X11_GREEN: Self = rgb(0.000, 1.000, 0.000);
    pub const LIMERICK: Self = rgb(0.616, 0.761, 0.035);
    pub const LINCOLN_GREEN: Self = rgb(0.098, 0.349, 0.020);
    pub const LINEN: Self = rgb(0.980, 0.941, 0.902);
    pub const LION: Self = rgb(0.757, 0.604, 0.420);
    pub const LITTLE_BOY_BLUE: Self = rgb(0.424, 0.627, 0.863);
    pub const LIVER: Self = rgb(0.325, 0.294, 0.310);
    pub const LUST: Self = rgb(0.902, 0.125, 0.125);
    pub const MAGENTA: Self = rgb(1.000, 0.000, 1.000);
    pub const MAGENTA_DYE: Self = rgb(0.792, 0.122, 0.482);
    pub const MAGENTA_PROCESS: Self = rgb(1.000, 0.000, 0.565);
    pub const MAGIC_MINT: Self = rgb(0.667, 0.941, 0.820);
    pub const MAGNOLIA: Self = rgb(0.973, 0.957, 1.000);
    pub const MAHOGANY: Self = rgb(0.753, 0.251, 0.000);
    pub const MAIZE: Self = rgb(0.984, 0.925, 0.365);
    pub const MAJORELLE_BLUE: Self = rgb(0.376, 0.314, 0.863);
    pub const MALACHITE: Self = rgb(0.043, 0.855, 0.318);
    pub const MANATEE: Self = rgb(0.592, 0.604, 0.667);
    pub const MANGO_TANGO: Self = rgb(1.000, 0.510, 0.263);
    pub const MANTIS: Self = rgb(0.455, 0.765, 0.396);
    pub const MARDI_GRAS: Self = rgb(0.533, 0.000, 0.522);
    pub const MAROON_CRAYOLA: Self = rgb(0.765, 0.129, 0.282);
    pub const MAROON_HTML_CSS: Self = rgb(0.502, 0.000, 0.000);
    pub const MAROON_X11: Self = rgb(0.690, 0.188, 0.376);
    pub const MAUVE: Self = rgb(0.878, 0.690, 1.000);
    pub const MAUVE_TAUPE: Self = rgb(0.569, 0.373, 0.427);
    pub const MAUVELOUS: Self = rgb(0.937, 0.596, 0.667);
    pub const MAYA_BLUE: Self = rgb(0.451, 0.761, 0.984);
    pub const MEAT_BROWN: Self = rgb(0.898, 0.718, 0.231);
    pub const MEDIUM_AQUAMARINE: Self = rgb(0.400, 0.867, 0.667);
    pub const MEDIUM_BLUE: Self = rgb(0.000, 0.000, 0.804);
    pub const MEDIUM_CANDY_APPLE_RED: Self = rgb(0.886, 0.024, 0.173);
    pub const MEDIUM_CARMINE: Self = rgb(0.686, 0.251, 0.208);
    pub const MEDIUM_CHAMPAGNE: Self = rgb(0.953, 0.898, 0.671);
    pub const MEDIUM_ELECTRIC_BLUE: Self = rgb(0.012, 0.314, 0.588);
    pub const MEDIUM_JUNGLE_GREEN: Self = rgb(0.110, 0.208, 0.176);
    pub const MEDIUM_LAVENDER_MAGENTA: Self = rgb(0.867, 0.627, 0.867);
    pub const MEDIUM_ORCHID: Self = rgb(0.729, 0.333, 0.827);
    pub const MEDIUM_PERSIAN_BLUE: Self = rgb(0.000, 0.404, 0.647);
    pub const MEDIUM_PURPLE: Self = rgb(0.576, 0.439, 0.859);
    pub const MEDIUM_RED_VIOLET: Self = rgb(0.733, 0.200, 0.522);
    pub const MEDIUM_RUBY: Self = rgb(0.667, 0.251, 0.412);
    pub const MEDIUM_SEA_GREEN: Self = rgb(0.235, 0.702, 0.443);
    pub const MEDIUM_SLATE_BLUE: Self = rgb(0.482, 0.408, 0.933);
    pub const MEDIUM_SPRING_BUD: Self = rgb(0.788, 0.863, 0.529);
    pub const MEDIUM_SPRING_GREEN: Self = rgb(0.000, 0.980, 0.604);
    pub const MEDIUM_TAUPE: Self = rgb(0.404, 0.298, 0.278);
    pub const MEDIUM_TURQUOISE: Self = rgb(0.282, 0.820, 0.800);
    pub const MEDIUM_TUSCAN_RED: Self = rgb(0.475, 0.267, 0.231);
    pub const MEDIUM_VERMILION: Self = rgb(0.851, 0.376, 0.231);
    pub const MEDIUM_VIOLET_RED: Self = rgb(0.780, 0.082, 0.522);
    pub const MELLOW_APRICOT: Self = rgb(0.973, 0.722, 0.471);
    pub const MELLOW_YELLOW: Self = rgb(0.973, 0.871, 0.494);
    pub const MELON: Self = rgb(0.992, 0.737, 0.706);
    pub const MIDNIGHT_BLUE: Self = rgb(0.098, 0.098, 0.439);
    pub const MIDNIGHT_GREEN_EAGLE_GREEN: Self = rgb(0.000, 0.286, 0.325);
    pub const MIKADO_YELLOW: Self = rgb(1.000, 0.769, 0.047);
    pub const MINT: Self = rgb(0.243, 0.706, 0.537);
    pub const MINT_CREAM: Self = rgb(0.961, 1.000, 0.980);
    pub const MINT_GREEN: Self = rgb(0.596, 1.000, 0.596);
    pub const MISTY_ROSE: Self = rgb(1.000, 0.894, 0.882);
    pub const MOCCASIN: Self = rgb(0.980, 0.922, 0.843);
    pub const MODE_BEIGE: Self = rgb(0.588, 0.443, 0.090);
    pub const MOONSTONE_BLUE: Self = rgb(0.451, 0.663, 0.761);
    pub const MORDANT_RED_19: Self = rgb(0.682, 0.047, 0.000);
    pub const MOSS_GREEN: Self = rgb(0.678, 0.875, 0.678);
    pub const MOUNTAIN_MEADOW: Self = rgb(0.188, 0.729, 0.561);
    pub const MOUNTBATTEN_PINK: Self = rgb(0.600, 0.478, 0.553);
    pub const MSU_GREEN: Self = rgb(0.094, 0.271, 0.231);
    pub const MULBERRY: Self = rgb(0.773, 0.294, 0.549);
    pub const MUSTARD: Self = rgb(1.000, 0.859, 0.345);
    pub const MYRTLE: Self = rgb(0.129, 0.259, 0.118);
    pub const NADESHIKO_PINK: Self = rgb(0.965, 0.678, 0.776);
    pub const NAPIER_GREEN: Self = rgb(0.165, 0.502, 0.000);
    pub const NAPLES_YELLOW: Self = rgb(0.980, 0.855, 0.369);
    pub const NAVAJO_WHITE: Self = rgb(1.000, 0.871, 0.678);
    pub const NAVY_BLUE: Self = rgb(0.000, 0.000, 0.502);
    pub const NEON_CARROT: Self = rgb(1.000, 0.639, 0.263);
    pub const NEON_FUCHSIA: Self = rgb(0.996, 0.255, 0.392);
    pub const NEON_GREEN: Self = rgb(0.224, 1.000, 0.078);
    pub const NEW_YORK_PINK: Self = rgb(0.843, 0.514, 0.498);
    pub const NON_PHOTO_BLUE: Self = rgb(0.643, 0.867, 0.929);
    pub const NORTH_TEXAS_GREEN: Self = rgb(0.020, 0.565, 0.200);
    pub const OCEAN_BOAT_BLUE: Self = rgb(0.000, 0.467, 0.745);
    pub const OCHRE: Self = rgb(0.800, 0.467, 0.133);
    pub const OFFICE_GREEN: Self = rgb(0.000, 0.502, 0.000);
    pub const OLD_GOLD: Self = rgb(0.812, 0.710, 0.231);
    pub const OLD_LACE: Self = rgb(0.992, 0.961, 0.902);
    pub const OLD_LAVENDER: Self = rgb(0.475, 0.408, 0.471);
    pub const OLD_MAUVE: Self = rgb(0.404, 0.192, 0.278);
    pub const OLD_ROSE: Self = rgb(0.753, 0.502, 0.506);
    pub const OLIVE: Self = rgb(0.502, 0.502, 0.000);
    pub const OLIVE_DRAB_7: Self = rgb(0.235, 0.204, 0.122);
    pub const OLIVE_DRAB_WEB_OLIVE_DRAB_3: Self = rgb(0.420, 0.557, 0.137);
    pub const OLIVINE: Self = rgb(0.604, 0.725, 0.451);
    pub const ONYX: Self = rgb(0.208, 0.220, 0.224);
    pub const OPERA_MAUVE: Self = rgb(0.718, 0.518, 0.655);
    pub const ORANGE_COLOR_WHEEL: Self = rgb(1.000, 0.498, 0.000);
    pub const ORANGE_PEEL: Self = rgb(1.000, 0.624, 0.000);
    pub const ORANGE_RED: Self = rgb(1.000, 0.271, 0.000);
    pub const ORANGE_RYB: Self = rgb(0.984, 0.600, 0.008);
    pub const ORANGE_WEB_COLOR: Self = rgb(1.000, 0.647, 0.000);
    pub const ORCHID: Self = rgb(0.855, 0.439, 0.839);
    pub const OTTER_BROWN: Self = rgb(0.396, 0.263, 0.129);
    pub const OU_CRIMSON_RED: Self = rgb(0.600, 0.000, 0.000);
    pub const OUTER_SPACE: Self = rgb(0.255, 0.290, 0.298);
    pub const OUTRAGEOUS_ORANGE: Self = rgb(1.000, 0.431, 0.290);
    pub const OXFORD_BLUE: Self = rgb(0.000, 0.129, 0.278);
    pub const PAKISTAN_GREEN: Self = rgb(0.000, 0.400, 0.000);
    pub const PALATINATE_BLUE: Self = rgb(0.153, 0.231, 0.886);
    pub const PALATINATE_PURPLE: Self = rgb(0.408, 0.157, 0.376);
    pub const PALE_AQUA: Self = rgb(0.737, 0.831, 0.902);
    pub const PALE_BLUE: Self = rgb(0.686, 0.933, 0.933);
    pub const PALE_BROWN: Self = rgb(0.596, 0.463, 0.329);
    pub const PALE_CARMINE: Self = rgb(0.686, 0.251, 0.208);
    pub const PALE_CERULEAN: Self = rgb(0.608, 0.769, 0.886);
    pub const PALE_CHESTNUT: Self = rgb(0.867, 0.678, 0.686);
    pub const PALE_COPPER: Self = rgb(0.855, 0.541, 0.404);
    pub const PALE_CORNFLOWER_BLUE: Self = rgb(0.671, 0.804, 0.937);
    pub const PALE_GOLD: Self = rgb(0.902, 0.745, 0.541);
    pub const PALE_GOLDENROD: Self = rgb(0.933, 0.910, 0.667);
    pub const PALE_GREEN: Self = rgb(0.596, 0.984, 0.596);
    pub const PALE_LAVENDER: Self = rgb(0.863, 0.816, 1.000);
    pub const PALE_MAGENTA: Self = rgb(0.976, 0.518, 0.898);
    pub const PALE_PINK: Self = rgb(0.980, 0.855, 0.867);
    pub const PALE_PLUM: Self = rgb(0.867, 0.627, 0.867);
    pub const PALE_RED_VIOLET: Self = rgb(0.859, 0.439, 0.576);
    pub const PALE_ROBIN_EGG_BLUE: Self = rgb(0.588, 0.871, 0.820);
    pub const PALE_SILVER: Self = rgb(0.788, 0.753, 0.733);
    pub const PALE_SPRING_BUD: Self = rgb(0.925, 0.922, 0.741);
    pub const PALE_TAUPE: Self = rgb(0.737, 0.596, 0.494);
    pub const PALE_VIOLET_RED: Self = rgb(0.859, 0.439, 0.576);
    pub const PANSY_PURPLE: Self = rgb(0.471, 0.094, 0.290);
    pub const PAPAYA_WHIP: Self = rgb(1.000, 0.937, 0.835);
    pub const PARIS_GREEN: Self = rgb(0.314, 0.784, 0.471);
    pub const PASTEL_BLUE: Self = rgb(0.682, 0.776, 0.812);
    pub const PASTEL_BROWN: Self = rgb(0.514, 0.412, 0.325);
    pub const PASTEL_GRAY: Self = rgb(0.812, 0.812, 0.769);
    pub const PASTEL_GREEN: Self = rgb(0.467, 0.867, 0.467);
    pub const PASTEL_MAGENTA: Self = rgb(0.957, 0.604, 0.761);
    pub const PASTEL_ORANGE: Self = rgb(1.000, 0.702, 0.278);
    pub const PASTEL_PINK: Self = rgb(0.871, 0.647, 0.643);
    pub const PASTEL_PURPLE: Self = rgb(0.702, 0.620, 0.710);
    pub const PASTEL_RED: Self = rgb(1.000, 0.412, 0.380);
    pub const PASTEL_VIOLET: Self = rgb(0.796, 0.600, 0.788);
    pub const PASTEL_YELLOW: Self = rgb(0.992, 0.992, 0.588);
    pub const PATRIARCH: Self = rgb(0.502, 0.000, 0.502);
    pub const PAYNE_S_GREY: Self = rgb(0.325, 0.408, 0.471);
    pub const PEACH: Self = rgb(1.000, 0.898, 0.706);
    pub const PEACH_CRAYOLA: Self = rgb(1.000, 0.796, 0.643);
    pub const PEACH_ORANGE: Self = rgb(1.000, 0.800, 0.600);
    pub const PEACH_PUFF: Self = rgb(1.000, 0.855, 0.725);
    pub const PEACH_YELLOW: Self = rgb(0.980, 0.875, 0.678);
    pub const PEAR: Self = rgb(0.820, 0.886, 0.192);
    pub const PEARL: Self = rgb(0.918, 0.878, 0.784);
    pub const PEARL_AQUA: Self = rgb(0.533, 0.847, 0.753);
    pub const PEARLY_PURPLE: Self = rgb(0.718, 0.408, 0.635);
    pub const PERIDOT: Self = rgb(0.902, 0.886, 0.000);
    pub const PERIWINKLE: Self = rgb(0.800, 0.800, 1.000);
    pub const PERSIAN_BLUE: Self = rgb(0.110, 0.224, 0.733);
    pub const PERSIAN_GREEN: Self = rgb(0.000, 0.651, 0.576);
    pub const PERSIAN_INDIGO: Self = rgb(0.196, 0.071, 0.478);
    pub const PERSIAN_ORANGE: Self = rgb(0.851, 0.565, 0.345);
    pub const PERSIAN_PINK: Self = rgb(0.969, 0.498, 0.745);
    pub const PERSIAN_PLUM: Self = rgb(0.439, 0.110, 0.110);
    pub const PERSIAN_RED: Self = rgb(0.800, 0.200, 0.200);
    pub const PERSIAN_ROSE: Self = rgb(0.996, 0.157, 0.635);
    pub const PERSIMMON: Self = rgb(0.925, 0.345, 0.000);
    pub const PERU: Self = rgb(0.804, 0.522, 0.247);
    pub const PHLOX: Self = rgb(0.875, 0.000, 1.000);
    pub const PHTHALO_BLUE: Self = rgb(0.000, 0.059, 0.537);
    pub const PHTHALO_GREEN: Self = rgb(0.071, 0.208, 0.141);
    pub const PIGGY_PINK: Self = rgb(0.992, 0.867, 0.902);
    pub const PINE_GREEN: Self = rgb(0.004, 0.475, 0.435);
    pub const PINK: Self = rgb(1.000, 0.753, 0.796);
    pub const PINK_LACE: Self = rgb(1.000, 0.867, 0.957);
    pub const PINK_ORANGE: Self = rgb(1.000, 0.600, 0.400);
    pub const PINK_PEARL: Self = rgb(0.906, 0.675, 0.812);
    pub const PINK_SHERBET: Self = rgb(0.969, 0.561, 0.655);
    pub const PISTACHIO: Self = rgb(0.576, 0.773, 0.447);
    pub const PLATINUM: Self = rgb(0.898, 0.894, 0.886);
    pub const PLUM_TRADITIONAL: Self = rgb(0.557, 0.271, 0.522);
    pub const PLUM_WEB: Self = rgb(0.867, 0.627, 0.867);
    pub const PORTLAND_ORANGE: Self = rgb(1.000, 0.353, 0.212);
    pub const POWDER_BLUE_WEB: Self = rgb(0.690, 0.878, 0.902);
    pub const PRINCETON_ORANGE: Self = rgb(1.000, 0.561, 0.000);
    pub const PRUNE: Self = rgb(0.439, 0.110, 0.110);
    pub const PRUSSIAN_BLUE: Self = rgb(0.000, 0.192, 0.325);
    pub const PSYCHEDELIC_PURPLE: Self = rgb(0.875, 0.000, 1.000);
    pub const PUCE: Self = rgb(0.800, 0.533, 0.600);
    pub const PUMPKIN: Self = rgb(1.000, 0.459, 0.094);
    pub const PURPLE_HEART: Self = rgb(0.412, 0.208, 0.612);
    pub const PURPLE_HTML_CSS: Self = rgb(0.502, 0.000, 0.502);
    pub const PURPLE_MOUNTAIN_MAJESTY: Self = rgb(0.588, 0.471, 0.714);
    pub const PURPLE_MUNSELL: Self = rgb(0.624, 0.000, 0.773);
    pub const PURPLE_PIZZAZZ: Self = rgb(0.996, 0.306, 0.855);
    pub const PURPLE_TAUPE: Self = rgb(0.314, 0.251, 0.302);
    pub const PURPLE_X11: Self = rgb(0.627, 0.125, 0.941);
    pub const QUARTZ: Self = rgb(0.318, 0.282, 0.310);
    pub const RACKLEY: Self = rgb(0.365, 0.541, 0.659);
    pub const RADICAL_RED: Self = rgb(1.000, 0.208, 0.369);
    pub const RAJAH: Self = rgb(0.984, 0.671, 0.376);
    pub const RASPBERRY: Self = rgb(0.890, 0.043, 0.365);
    pub const RASPBERRY_GLACE: Self = rgb(0.569, 0.373, 0.427);
    pub const RASPBERRY_PINK: Self = rgb(0.886, 0.314, 0.596);
    pub const RASPBERRY_ROSE: Self = rgb(0.702, 0.267, 0.424);
    pub const RAW_UMBER: Self = rgb(0.510, 0.400, 0.267);
    pub const RAZZLE_DAZZLE_ROSE: Self = rgb(1.000, 0.200, 0.800);
    pub const RAZZMATAZZ: Self = rgb(0.890, 0.145, 0.420);
    pub const RED: Self = rgb(1.000, 0.000, 0.000);
    pub const RED_BROWN: Self = rgb(0.647, 0.165, 0.165);
    pub const RED_DEVIL: Self = rgb(0.525, 0.004, 0.067);
    pub const RED_MUNSELL: Self = rgb(0.949, 0.000, 0.235);
    pub const RED_NCS: Self = rgb(0.769, 0.008, 0.200);
    pub const RED_ORANGE: Self = rgb(1.000, 0.325, 0.286);
    pub const RED_PIGMENT: Self = rgb(0.929, 0.110, 0.141);
    pub const RED_RYB: Self = rgb(0.996, 0.153, 0.071);
    pub const RED_VIOLET: Self = rgb(0.780, 0.082, 0.522);
    pub const REDWOOD: Self = rgb(0.671, 0.306, 0.322);
    pub const REGALIA: Self = rgb(0.322, 0.176, 0.502);
    pub const RESOLUTION_BLUE: Self = rgb(0.000, 0.137, 0.529);
    pub const RICH_BLACK: Self = rgb(0.000, 0.251, 0.251);
    pub const RICH_BRILLIANT_LAVENDER: Self = rgb(0.945, 0.655, 0.996);
    pub const RICH_CARMINE: Self = rgb(0.843, 0.000, 0.251);
    pub const RICH_ELECTRIC_BLUE: Self = rgb(0.031, 0.573, 0.816);
    pub const RICH_LAVENDER: Self = rgb(0.655, 0.420, 0.812);
    pub const RICH_LILAC: Self = rgb(0.714, 0.400, 0.824);
    pub const RICH_MAROON: Self = rgb(0.690, 0.188, 0.376);
    pub const RIFLE_GREEN: Self = rgb(0.255, 0.282, 0.200);
    pub const ROBIN_EGG_BLUE: Self = rgb(0.000, 0.800, 0.800);
    pub const ROSE: Self = rgb(1.000, 0.000, 0.498);
    pub const ROSE_BONBON: Self = rgb(0.976, 0.259, 0.620);
    pub const ROSE_EBONY: Self = rgb(0.404, 0.282, 0.275);
    pub const ROSE_GOLD: Self = rgb(0.718, 0.431, 0.475);
    pub const ROSE_MADDER: Self = rgb(0.890, 0.149, 0.212);
    pub const ROSE_PINK: Self = rgb(1.000, 0.400, 0.800);
    pub const ROSE_QUARTZ: Self = rgb(0.667, 0.596, 0.663);
    pub const ROSE_TAUPE: Self = rgb(0.565, 0.365, 0.365);
    pub const ROSE_VALE: Self = rgb(0.671, 0.306, 0.322);
    pub const ROSEWOOD: Self = rgb(0.396, 0.000, 0.043);
    pub const ROSSO_CORSA: Self = rgb(0.831, 0.000, 0.000);
    pub const ROSY_BROWN: Self = rgb(0.737, 0.561, 0.561);
    pub const ROYAL_AZURE: Self = rgb(0.000, 0.220, 0.659);
    pub const ROYAL_BLUE_TRADITIONAL: Self = rgb(0.000, 0.137, 0.400);
    pub const ROYAL_BLUE_WEB: Self = rgb(0.255, 0.412, 0.882);
    pub const ROYAL_FUCHSIA: Self = rgb(0.792, 0.173, 0.573);
    pub const ROYAL_PURPLE: Self = rgb(0.471, 0.318, 0.663);
    pub const ROYAL_YELLOW: Self = rgb(0.980, 0.855, 0.369);
    pub const RUBINE_RED: Self = rgb(0.820, 0.000, 0.337);
    pub const RUBY: Self = rgb(0.878, 0.067, 0.373);
    pub const RUBY_RED: Self = rgb(0.608, 0.067, 0.118);
    pub const RUDDY: Self = rgb(1.000, 0.000, 0.157);
    pub const RUDDY_BROWN: Self = rgb(0.733, 0.396, 0.157);
    pub const RUDDY_PINK: Self = rgb(0.882, 0.557, 0.588);
    pub const RUFOUS: Self = rgb(0.659, 0.110, 0.027);
    pub const RUSSET: Self = rgb(0.502, 0.275, 0.106);
    pub const RUST: Self = rgb(0.718, 0.255, 0.055);
    pub const RUSTY_RED: Self = rgb(0.855, 0.173, 0.263);
    pub const SACRAMENTO_STATE_GREEN: Self = rgb(0.000, 0.337, 0.247);
    pub const SADDLE_BROWN: Self = rgb(0.545, 0.271, 0.075);
    pub const SAFETY_ORANGE_BLAZE_ORANGE: Self = rgb(1.000, 0.404, 0.000);
    pub const SAFFRON: Self = rgb(0.957, 0.769, 0.188);
    pub const SALMON: Self = rgb(1.000, 0.549, 0.412);
    pub const SALMON_PINK: Self = rgb(1.000, 0.569, 0.643);
    pub const SAND: Self = rgb(0.761, 0.698, 0.502);
    pub const SAND_DUNE: Self = rgb(0.588, 0.443, 0.090);
    pub const SANDSTORM: Self = rgb(0.925, 0.835, 0.251);
    pub const SANDY_BROWN: Self = rgb(0.957, 0.643, 0.376);
    pub const SANDY_TAUPE: Self = rgb(0.588, 0.443, 0.090);
    pub const SANGRIA: Self = rgb(0.573, 0.000, 0.039);
    pub const SAP_GREEN: Self = rgb(0.314, 0.490, 0.165);
    pub const SAPPHIRE: Self = rgb(0.059, 0.322, 0.729);
    pub const SAPPHIRE_BLUE: Self = rgb(0.000, 0.404, 0.647);
    pub const SATIN_SHEEN_GOLD: Self = rgb(0.796, 0.631, 0.208);
    pub const SCARLET: Self = rgb(1.000, 0.141, 0.000);
    pub const SCARLET_CRAYOLA: Self = rgb(0.992, 0.055, 0.208);
    pub const SCHOOL_BUS_YELLOW: Self = rgb(1.000, 0.847, 0.000);
    pub const SCREAMIN_GREEN: Self = rgb(0.463, 1.000, 0.478);
    pub const SEA_BLUE: Self = rgb(0.000, 0.412, 0.580);
    pub const SEA_GREEN: Self = rgb(0.180, 0.545, 0.341);
    pub const SEAL_BROWN: Self = rgb(0.196, 0.078, 0.078);
    pub const SEASHELL: Self = rgb(1.000, 0.961, 0.933);
    pub const SELECTIVE_YELLOW: Self = rgb(1.000, 0.729, 0.000);
    pub const SEPIA: Self = rgb(0.439, 0.259, 0.078);
    pub const SHADOW: Self = rgb(0.541, 0.475, 0.365);
    pub const SHAMROCK_GREEN: Self = rgb(0.000, 0.620, 0.376);
    pub const SHOCKING_PINK: Self = rgb(0.988, 0.059, 0.753);
    pub const SHOCKING_PINK_CRAYOLA: Self = rgb(1.000, 0.435, 1.000);
    pub const SIENNA: Self = rgb(0.533, 0.176, 0.090);
    pub const SILVER: Self = rgb(0.753, 0.753, 0.753);
    pub const SINOPIA: Self = rgb(0.796, 0.255, 0.043);
    pub const SKOBELOFF: Self = rgb(0.000, 0.455, 0.455);
    pub const SKY_BLUE: Self = rgb(0.529, 0.808, 0.922);
    pub const SKY_MAGENTA: Self = rgb(0.812, 0.443, 0.686);
    pub const SLATE_BLUE: Self = rgb(0.416, 0.353, 0.804);
    pub const SLATE_GRAY: Self = rgb(0.439, 0.502, 0.565);
    pub const SMALT_DARK_POWDER_BLUE: Self = rgb(0.000, 0.200, 0.600);
    pub const SMOKEY_TOPAZ: Self = rgb(0.576, 0.239, 0.255);
    pub const SMOKY_BLACK: Self = rgb(0.063, 0.047, 0.031);
    pub const SNOW: Self = rgb(1.000, 0.980, 0.980);
    pub const SPIRO_DISCO_BALL: Self = rgb(0.059, 0.753, 0.988);
    pub const SPRING_BUD: Self = rgb(0.655, 0.988, 0.000);
    pub const SPRING_GREEN: Self = rgb(0.000, 1.000, 0.498);
    pub const ST_PATRICK_S_BLUE: Self = rgb(0.137, 0.161, 0.478);
    pub const STEEL_BLUE: Self = rgb(0.275, 0.510, 0.706);
    pub const STIL_DE_GRAIN_YELLOW: Self = rgb(0.980, 0.855, 0.369);
    pub const STIZZA: Self = rgb(0.600, 0.000, 0.000);
    pub const STORMCLOUD: Self = rgb(0.310, 0.400, 0.416);
    pub const STRAW: Self = rgb(0.894, 0.851, 0.435);
    pub const SUNGLOW: Self = rgb(1.000, 0.800, 0.200);
    pub const SUNSET: Self = rgb(0.980, 0.839, 0.647);
    pub const TAN: Self = rgb(0.824, 0.706, 0.549);
    pub const TANGELO: Self = rgb(0.976, 0.302, 0.000);
    pub const TANGERINE: Self = rgb(0.949, 0.522, 0.000);
    pub const TANGERINE_YELLOW: Self = rgb(1.000, 0.800, 0.000);
    pub const TANGO_PINK: Self = rgb(0.894, 0.443, 0.478);
    pub const TAUPE: Self = rgb(0.282, 0.235, 0.196);
    pub const TAUPE_GRAY: Self = rgb(0.545, 0.522, 0.537);
    pub const TEA_GREEN: Self = rgb(0.816, 0.941, 0.753);
    pub const TEA_ROSE_ORANGE: Self = rgb(0.973, 0.514, 0.475);
    pub const TEA_ROSE_ROSE: Self = rgb(0.957, 0.761, 0.761);
    pub const TEAL: Self = rgb(0.000, 0.502, 0.502);
    pub const TEAL_BLUE: Self = rgb(0.212, 0.459, 0.533);
    pub const TEAL_GREEN: Self = rgb(0.000, 0.510, 0.498);
    pub const TELEMAGENTA: Self = rgb(0.812, 0.204, 0.463);
    pub const TENN_TAWNY: Self = rgb(0.804, 0.341, 0.000);
    pub const TERRA_COTTA: Self = rgb(0.886, 0.447, 0.357);
    pub const THISTLE: Self = rgb(0.847, 0.749, 0.847);
    pub const THULIAN_PINK: Self = rgb(0.871, 0.435, 0.631);
    pub const TICKLE_ME_PINK: Self = rgb(0.988, 0.537, 0.675);
    pub const TIFFANY_BLUE: Self = rgb(0.039, 0.729, 0.710);
    pub const TIGER_S_EYE: Self = rgb(0.878, 0.553, 0.235);
    pub const TIMBERWOLF: Self = rgb(0.859, 0.843, 0.824);
    pub const TITANIUM_YELLOW: Self = rgb(0.933, 0.902, 0.000);
    pub const TOMATO: Self = rgb(1.000, 0.388, 0.278);
    pub const TOOLBOX: Self = rgb(0.455, 0.424, 0.753);
    pub const TOPAZ: Self = rgb(1.000, 0.784, 0.486);
    pub const TRACTOR_RED: Self = rgb(0.992, 0.055, 0.208);
    pub const TROLLEY_GREY: Self = rgb(0.502, 0.502, 0.502);
    pub const TROPICAL_RAIN_FOREST: Self = rgb(0.000, 0.459, 0.369);
    pub const TRUE_BLUE: Self = rgb(0.000, 0.451, 0.812);
    pub const TUFTS_BLUE: Self = rgb(0.255, 0.490, 0.757);
    pub const TUMBLEWEED: Self = rgb(0.871, 0.667, 0.533);
    pub const TURKISH_ROSE: Self = rgb(0.710, 0.447, 0.506);
    pub const TURQUOISE: Self = rgb(0.188, 0.835, 0.784);
    pub const TURQUOISE_BLUE: Self = rgb(0.000, 1.000, 0.937);
    pub const TURQUOISE_GREEN: Self = rgb(0.627, 0.839, 0.706);
    pub const TUSCAN_RED: Self = rgb(0.486, 0.282, 0.282);
    pub const TWILIGHT_LAVENDER: Self = rgb(0.541, 0.286, 0.420);
    pub const TYRIAN_PURPLE: Self = rgb(0.400, 0.008, 0.235);
    pub const UA_BLUE: Self = rgb(0.000, 0.200, 0.667);
    pub const UA_RED: Self = rgb(0.851, 0.000, 0.298);
    pub const UBE: Self = rgb(0.533, 0.471, 0.765);
    pub const UCLA_BLUE: Self = rgb(0.325, 0.408, 0.584);
    pub const UCLA_GOLD: Self = rgb(1.000, 0.702, 0.000);
    pub const UFO_GREEN: Self = rgb(0.235, 0.816, 0.439);
    pub const ULTRA_PINK: Self = rgb(1.000, 0.435, 1.000);
    pub const ULTRAMARINE: Self = rgb(0.071, 0.039, 0.561);
    pub const ULTRAMARINE_BLUE: Self = rgb(0.255, 0.400, 0.961);
    pub const UMBER: Self = rgb(0.388, 0.318, 0.278);
    pub const UNBLEACHED_SILK: Self = rgb(1.000, 0.867, 0.792);
    pub const UNITED_NATIONS_BLUE: Self = rgb(0.357, 0.573, 0.898);
    pub const UNIVERSITY_OF_CALIFORNIA_GOLD: Self = rgb(0.718, 0.529, 0.153);
    pub const UNMELLOW_YELLOW: Self = rgb(1.000, 1.000, 0.400);
    pub const UP_FOREST_GREEN: Self = rgb(0.004, 0.267, 0.129);
    pub const UP_MAROON: Self = rgb(0.482, 0.067, 0.075);
    pub const UPSDELL_RED: Self = rgb(0.682, 0.125, 0.161);
    pub const UROBILIN: Self = rgb(0.882, 0.678, 0.129);
    pub const USAFA_BLUE: Self = rgb(0.000, 0.310, 0.596);
    pub const USC_CARDINAL: Self = rgb(0.600, 0.000, 0.000);
    pub const USC_GOLD: Self = rgb(1.000, 0.800, 0.000);
    pub const UTAH_CRIMSON: Self = rgb(0.827, 0.000, 0.247);
    pub const VANILLA: Self = rgb(0.953, 0.898, 0.671);
    pub const VEGAS_GOLD: Self = rgb(0.773, 0.702, 0.345);
    pub const VENETIAN_RED: Self = rgb(0.784, 0.031, 0.082);
    pub const VERDIGRIS: Self = rgb(0.263, 0.702, 0.682);
    pub const VERMILION_CINNABAR: Self = rgb(0.890, 0.259, 0.204);
    pub const VERMILION_PLOCHERE: Self = rgb(0.851, 0.376, 0.231);
    pub const VERONICA: Self = rgb(0.627, 0.125, 0.941);
    pub const VIOLET: Self = rgb(0.561, 0.000, 1.000);
    pub const VIOLET_BLUE: Self = rgb(0.196, 0.290, 0.698);
    pub const VIOLET_COLOR_WHEEL: Self = rgb(0.498, 0.000, 1.000);
    pub const VIOLET_RYB: Self = rgb(0.525, 0.004, 0.686);
    pub const VIOLET_WEB: Self = rgb(0.933, 0.510, 0.933);
    pub const VIRIDIAN: Self = rgb(0.251, 0.510, 0.427);
    pub const VIVID_AUBURN: Self = rgb(0.573, 0.153, 0.141);
    pub const VIVID_BURGUNDY: Self = rgb(0.624, 0.114, 0.208);
    pub const VIVID_CERISE: Self = rgb(0.855, 0.114, 0.506);
    pub const VIVID_TANGERINE: Self = rgb(1.000, 0.627, 0.537);
    pub const VIVID_VIOLET: Self = rgb(0.624, 0.000, 1.000);
    pub const WARM_BLACK: Self = rgb(0.000, 0.259, 0.259);
    pub const WATERSPOUT: Self = rgb(0.643, 0.957, 0.976);
    pub const WENGE: Self = rgb(0.392, 0.329, 0.322);
    pub const WHEAT: Self = rgb(0.961, 0.871, 0.702);
    pub const WHITE: Self = rgb(1.000, 1.000, 1.000);
    pub const WHITE_SMOKE: Self = rgb(0.961, 0.961, 0.961);
    pub const WILD_BLUE_YONDER: Self = rgb(0.635, 0.678, 0.816);
    pub const WILD_STRAWBERRY: Self = rgb(1.000, 0.263, 0.643);
    pub const WILD_WATERMELON: Self = rgb(0.988, 0.424, 0.522);
    pub const WINE: Self = rgb(0.447, 0.184, 0.216);
    pub const WINE_DREGS: Self = rgb(0.404, 0.192, 0.278);
    pub const WISTERIA: Self = rgb(0.788, 0.627, 0.863);
    pub const WOOD_BROWN: Self = rgb(0.757, 0.604, 0.420);
    pub const XANADU: Self = rgb(0.451, 0.525, 0.471);
    pub const YALE_BLUE: Self = rgb(0.059, 0.302, 0.573);
    pub const YELLOW: Self = rgb(1.000, 1.000, 0.000);
    pub const YELLOW_GREEN: Self = rgb(0.604, 0.804, 0.196);
    pub const YELLOW_MUNSELL: Self = rgb(0.937, 0.800, 0.000);
    pub const YELLOW_NCS: Self = rgb(1.000, 0.827, 0.000);
    pub const YELLOW_ORANGE: Self = rgb(1.000, 0.682, 0.259);
    pub const YELLOW_PROCESS: Self = rgb(1.000, 0.937, 0.000);
    pub const YELLOW_RYB: Self = rgb(0.996, 0.996, 0.200);
    pub const ZAFFRE: Self = rgb(0.000, 0.078, 0.659);
    pub const ZINNWALDITE_BROWN: Self = rgb(0.173, 0.086, 0.031);

    pub fn all_approx_one_or_zero(&self) -> bool {
        const ZERO_THRESHOLD: f32 = 0.5 / 255.0;
        const ONE_THRESHOLD: f32 = 254.5 / 255.0;
        (self.r < ZERO_THRESHOLD || self.r > ONE_THRESHOLD)
            && (self.g < ZERO_THRESHOLD || self.g > ONE_THRESHOLD)
            && (self.b < ZERO_THRESHOLD || self.b > ONE_THRESHOLD)
    }

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn clamped(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    pub fn create_image(&self) -> image::DynamicImage {
        self.with_a(1.0).create_image()
    }

    pub fn with_a(&self, a: f32) -> Rgba {
        rgba(self.r, self.g, self.b, a)
    }

    pub fn as_u8s(&self) -> [u8; 3] {
        [
            Self::f32_to_u8(self.r),
            Self::f32_to_u8(self.g),
            Self::f32_to_u8(self.b),
        ]
    }

    pub fn as_f32s(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    fn f32_to_u8(val: f32) -> u8 {
        (val * 255.0).round() as u8
    }

    /// Convert a normal vector (as from a normal map texture) into RGB
    /// colorspace.
    pub fn from_normal_vec(vec: Vec3) -> Self {
        let rgb_vec = (vec + vec3(1.0, 1.0, 1.0)) / 2.0;
        Self::new(rgb_vec.x as f32, rgb_vec.y as f32, rgb_vec.z as f32)
    }
}
pub const fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::new(r, g, b)
}

impl_op_ex_commutative!(*|c: Rgb, s: f32| -> Rgb { rgb(c.r * s, c.g * s, c.b * s) });
