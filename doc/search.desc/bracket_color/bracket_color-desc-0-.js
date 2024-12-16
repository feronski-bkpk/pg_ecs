searchState.loadedDescShard("bracket_color", 0, "This crate is part of the <code>bracket-lib</code> family.\nImport color pair support\nImport HSV color support\nImport Lerp as an iterator\nImport library of named colors\nImport Palette support\nExports the color functions/types in the <code>prelude</code> namespace.\nImport RGB color support\nImport RGBA color support\nRepresents two colors together, a foreground and a …\nThe background color\nThe foreground color\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new <code>ColorPair</code>, from two given colors.\nRepresents an H/S/V triplet, in the range 0..1 (32-bit …\nReturns the argument unchanged.\nConstructs a new HSV color, from 3 32-bit floats\nHue (range 0..1)\nCalls <code>U::from(self)</code>.\nProgress smoothly between two colors, in the HSV color …\nConstructs a new, zeroed (black) HSV triplet.\nSaturation (range 0..1)\nConverts an HSV triple to an RGB triple\nConverts to an RGBA value with a specified alpha level\nValue (range 0..1)\nImplements an Alpha-Only Lerp as an iterator\nAn HSV Lerp - transition from one HSV color to another in …\nImplements an RGB Lerp as an iterator\nImplements an RGBA Lerp as an iterator\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the <code>n_steps</code> component of the iterator\nCreates a new RGB lerp iterator. The iterator smoothly …\nCreates a new <code>HsvLerp</code> iterator.\nCreates a new RGB iterator\nCreates a new RGB iterator\nReturns the next step in the iterator\nReturns the next Lerp step\nReturns the next step in the iterator\nReturns the next step in the iterator\nInsert all named W3C colors into the palette\nEmpties the palette\nRetrieve a palette color by name from the global registry. …\nRegister a palette color by name with the global registry.\nError message type when failing to convert a hex code to …\nAn unexpected character (not #, A-F) was detected in the …\nThe HTML string was not a valid length. (Expects #AABBCC)\nNo # was included in the string.\nRepresents an R/G/B triplet, in the range 0..1 (32-bit …\nThe blue component (0..1)\nApplies a lengthier desaturate (via HSV) to the color\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a new RGB color, from 3 32-bit floats in the …\nConstructs from an HTML color code (e.g. “#eeffee”)\nConstructs a new RGB color, from 3 bytes in the range …\nThe green components (0..1)\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLerps by a specified percentage (from 0 to 1) between this …\nConstruct an RGB color from a tuple of u8, or a named …\nConstructs a new, zeroed (black) RGB triplet.\nThe red component (0..1)\nApplies a quick grayscale conversion to the color\nConverts an RGB triple to an HSV triple.\nConverts an RGB to an RGBA\nRepresents an R/G/B triplet, in the range 0..1 (32-bit …\nThe alpha component (0..1), 0 is transparent, 1 is solid\nThe blue component (0..1)\nApplies a lengthier desaturate (via HSV) to the color\nReturns the argument unchanged.\nConstructs a new RGB color, from 3 32-bit floats in the …\nConstructs from an HTML color code (e.g. “#eeffeeff”)\nConstructs a new RGB color, from 3 bytes in the range …\nThe green component (0..1)\nCalls <code>U::from(self)</code>.\nLerps by a specified percentage (from 0 to 1) between this …\nLerps only the alpha channel, by a specified percentage …\nConstruct an RGB color from a tuple of u8, or a named …\nConstructs a new, zeroed (black) RGB triplet.\nThe red component (0..1)\nApplies a quick grayscale conversion to the color\nConverts to an RGB, dropping the alpha component")