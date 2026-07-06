// Fcitx5 keysym constants.
// Auto-generated from fcitx-utils/keysymgen.h — do not edit manually.
// SPDX-License-Identifier: LGPL-2.1-or-later

#![allow(dead_code, non_upper_case_globals)]

pub const FCITX_KEY_None: u32 = 0x0;

/// Void symbol
pub const FCITX_KEY_VoidSymbol: u32 = 0xffffff;

/// U+0008 BACKSPACE
pub const FCITX_KEY_BackSpace: u32 = 0xff08;

/// U+0009 CHARACTER TABULATION
pub const FCITX_KEY_Tab: u32 = 0xff09;

/// U+000A LINE FEED
pub const FCITX_KEY_Linefeed: u32 = 0xff0a;

/// U+000B LINE TABULATION
pub const FCITX_KEY_Clear: u32 = 0xff0b;

/// U+000D CARRIAGE RETURN
pub const FCITX_KEY_Return: u32 = 0xff0d;

/// Pause, hold
pub const FCITX_KEY_Pause: u32 = 0xff13;

pub const FCITX_KEY_Scroll_Lock: u32 = 0xff14;

pub const FCITX_KEY_Sys_Req: u32 = 0xff15;

/// U+001B ESCAPE
pub const FCITX_KEY_Escape: u32 = 0xff1b;

/// U+007F DELETE
pub const FCITX_KEY_Delete: u32 = 0xffff;

/// Multi-key character compose
pub const FCITX_KEY_Multi_key: u32 = 0xff20;

pub const FCITX_KEY_Codeinput: u32 = 0xff37;

pub const FCITX_KEY_SingleCandidate: u32 = 0xff3c;

pub const FCITX_KEY_MultipleCandidate: u32 = 0xff3d;

pub const FCITX_KEY_PreviousCandidate: u32 = 0xff3e;

/// Kanji, Kanji convert
pub const FCITX_KEY_Kanji: u32 = 0xff21;

/// Cancel Conversion
pub const FCITX_KEY_Muhenkan: u32 = 0xff22;

/// Start/Stop Conversion
pub const FCITX_KEY_Henkan_Mode: u32 = 0xff23;

/// non-deprecated alias for Henkan_Mode
pub const FCITX_KEY_Henkan: u32 = 0xff23;

/// to Romaji
pub const FCITX_KEY_Romaji: u32 = 0xff24;

/// to Hiragana
pub const FCITX_KEY_Hiragana: u32 = 0xff25;

/// to Katakana
pub const FCITX_KEY_Katakana: u32 = 0xff26;

/// Hiragana/Katakana toggle
pub const FCITX_KEY_Hiragana_Katakana: u32 = 0xff27;

/// to Zenkaku
pub const FCITX_KEY_Zenkaku: u32 = 0xff28;

/// to Hankaku
pub const FCITX_KEY_Hankaku: u32 = 0xff29;

/// Zenkaku/Hankaku toggle
pub const FCITX_KEY_Zenkaku_Hankaku: u32 = 0xff2a;

/// Add to Dictionary
pub const FCITX_KEY_Touroku: u32 = 0xff2b;

/// Delete from Dictionary
pub const FCITX_KEY_Massyo: u32 = 0xff2c;

/// Kana Lock
pub const FCITX_KEY_Kana_Lock: u32 = 0xff2d;

/// Kana Shift
pub const FCITX_KEY_Kana_Shift: u32 = 0xff2e;

/// Alphanumeric Shift
pub const FCITX_KEY_Eisu_Shift: u32 = 0xff2f;

/// Alphanumeric toggle
pub const FCITX_KEY_Eisu_toggle: u32 = 0xff30;

/// Codeinput
pub const FCITX_KEY_Kanji_Bangou: u32 = 0xff37;

/// Multiple/All Candidate(s)
pub const FCITX_KEY_Zen_Koho: u32 = 0xff3d;

/// Previous Candidate
pub const FCITX_KEY_Mae_Koho: u32 = 0xff3e;

pub const FCITX_KEY_Home: u32 = 0xff50;

/// Move left, left arrow
pub const FCITX_KEY_Left: u32 = 0xff51;

/// Move up, up arrow
pub const FCITX_KEY_Up: u32 = 0xff52;

/// Move right, right arrow
pub const FCITX_KEY_Right: u32 = 0xff53;

/// Move down, down arrow
pub const FCITX_KEY_Down: u32 = 0xff54;

/// Prior, previous
pub const FCITX_KEY_Prior: u32 = 0xff55;

/// deprecated alias for Prior
pub const FCITX_KEY_Page_Up: u32 = 0xff55;

/// Next
pub const FCITX_KEY_Next: u32 = 0xff56;

/// deprecated alias for Next
pub const FCITX_KEY_Page_Down: u32 = 0xff56;

/// EOL
pub const FCITX_KEY_End: u32 = 0xff57;

/// BOL
pub const FCITX_KEY_Begin: u32 = 0xff58;

/// Select, mark
pub const FCITX_KEY_Select: u32 = 0xff60;

pub const FCITX_KEY_Print: u32 = 0xff61;

/// Execute, run, do
pub const FCITX_KEY_Execute: u32 = 0xff62;

/// Insert, insert here
pub const FCITX_KEY_Insert: u32 = 0xff63;

pub const FCITX_KEY_Undo: u32 = 0xff65;

/// Redo, again
pub const FCITX_KEY_Redo: u32 = 0xff66;

pub const FCITX_KEY_Menu: u32 = 0xff67;

/// Find, search
pub const FCITX_KEY_Find: u32 = 0xff68;

/// Cancel, stop, abort, exit
pub const FCITX_KEY_Cancel: u32 = 0xff69;

/// Help
pub const FCITX_KEY_Help: u32 = 0xff6a;

pub const FCITX_KEY_Break: u32 = 0xff6b;

/// Character set switch
pub const FCITX_KEY_Mode_switch: u32 = 0xff7e;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_script_switch: u32 = 0xff7e;

pub const FCITX_KEY_Num_Lock: u32 = 0xff7f;

/// <U+0020 SPACE>
pub const FCITX_KEY_KP_Space: u32 = 0xff80;

/// <U+0009 CHARACTER TABULATION>
pub const FCITX_KEY_KP_Tab: u32 = 0xff89;

/// <U+000D CARRIAGE RETURN>
pub const FCITX_KEY_KP_Enter: u32 = 0xff8d;

/// PF1, KP_A, ...
pub const FCITX_KEY_KP_F1: u32 = 0xff91;

pub const FCITX_KEY_KP_F2: u32 = 0xff92;

pub const FCITX_KEY_KP_F3: u32 = 0xff93;

pub const FCITX_KEY_KP_F4: u32 = 0xff94;

pub const FCITX_KEY_KP_Home: u32 = 0xff95;

pub const FCITX_KEY_KP_Left: u32 = 0xff96;

pub const FCITX_KEY_KP_Up: u32 = 0xff97;

pub const FCITX_KEY_KP_Right: u32 = 0xff98;

pub const FCITX_KEY_KP_Down: u32 = 0xff99;

pub const FCITX_KEY_KP_Prior: u32 = 0xff9a;

/// deprecated alias for KP_Prior
pub const FCITX_KEY_KP_Page_Up: u32 = 0xff9a;

pub const FCITX_KEY_KP_Next: u32 = 0xff9b;

/// deprecated alias for KP_Next
pub const FCITX_KEY_KP_Page_Down: u32 = 0xff9b;

pub const FCITX_KEY_KP_End: u32 = 0xff9c;

pub const FCITX_KEY_KP_Begin: u32 = 0xff9d;

pub const FCITX_KEY_KP_Insert: u32 = 0xff9e;

pub const FCITX_KEY_KP_Delete: u32 = 0xff9f;

/// <U+003D EQUALS SIGN>
pub const FCITX_KEY_KP_Equal: u32 = 0xffbd;

/// <U+002A ASTERISK>
pub const FCITX_KEY_KP_Multiply: u32 = 0xffaa;

/// <U+002B PLUS SIGN>
pub const FCITX_KEY_KP_Add: u32 = 0xffab;

/// <U+002C COMMA>
pub const FCITX_KEY_KP_Separator: u32 = 0xffac;

/// <U+002D HYPHEN-MINUS>
pub const FCITX_KEY_KP_Subtract: u32 = 0xffad;

/// <U+002E FULL STOP>
pub const FCITX_KEY_KP_Decimal: u32 = 0xffae;

/// <U+002F SOLIDUS>
pub const FCITX_KEY_KP_Divide: u32 = 0xffaf;

/// <U+0030 DIGIT ZERO>
pub const FCITX_KEY_KP_0: u32 = 0xffb0;

/// <U+0031 DIGIT ONE>
pub const FCITX_KEY_KP_1: u32 = 0xffb1;

/// <U+0032 DIGIT TWO>
pub const FCITX_KEY_KP_2: u32 = 0xffb2;

/// <U+0033 DIGIT THREE>
pub const FCITX_KEY_KP_3: u32 = 0xffb3;

/// <U+0034 DIGIT FOUR>
pub const FCITX_KEY_KP_4: u32 = 0xffb4;

/// <U+0035 DIGIT FIVE>
pub const FCITX_KEY_KP_5: u32 = 0xffb5;

/// <U+0036 DIGIT SIX>
pub const FCITX_KEY_KP_6: u32 = 0xffb6;

/// <U+0037 DIGIT SEVEN>
pub const FCITX_KEY_KP_7: u32 = 0xffb7;

/// <U+0038 DIGIT EIGHT>
pub const FCITX_KEY_KP_8: u32 = 0xffb8;

/// <U+0039 DIGIT NINE>
pub const FCITX_KEY_KP_9: u32 = 0xffb9;

pub const FCITX_KEY_F1: u32 = 0xffbe;

pub const FCITX_KEY_F2: u32 = 0xffbf;

pub const FCITX_KEY_F3: u32 = 0xffc0;

pub const FCITX_KEY_F4: u32 = 0xffc1;

pub const FCITX_KEY_F5: u32 = 0xffc2;

pub const FCITX_KEY_F6: u32 = 0xffc3;

pub const FCITX_KEY_F7: u32 = 0xffc4;

pub const FCITX_KEY_F8: u32 = 0xffc5;

pub const FCITX_KEY_F9: u32 = 0xffc6;

pub const FCITX_KEY_F10: u32 = 0xffc7;

pub const FCITX_KEY_F11: u32 = 0xffc8;

/// deprecated alias for F11
pub const FCITX_KEY_L1: u32 = 0xffc8;

pub const FCITX_KEY_F12: u32 = 0xffc9;

/// deprecated alias for F12
pub const FCITX_KEY_L2: u32 = 0xffc9;

pub const FCITX_KEY_F13: u32 = 0xffca;

/// deprecated alias for F13
pub const FCITX_KEY_L3: u32 = 0xffca;

pub const FCITX_KEY_F14: u32 = 0xffcb;

/// deprecated alias for F14
pub const FCITX_KEY_L4: u32 = 0xffcb;

pub const FCITX_KEY_F15: u32 = 0xffcc;

/// deprecated alias for F15
pub const FCITX_KEY_L5: u32 = 0xffcc;

pub const FCITX_KEY_F16: u32 = 0xffcd;

/// deprecated alias for F16
pub const FCITX_KEY_L6: u32 = 0xffcd;

pub const FCITX_KEY_F17: u32 = 0xffce;

/// deprecated alias for F17
pub const FCITX_KEY_L7: u32 = 0xffce;

pub const FCITX_KEY_F18: u32 = 0xffcf;

/// deprecated alias for F18
pub const FCITX_KEY_L8: u32 = 0xffcf;

pub const FCITX_KEY_F19: u32 = 0xffd0;

/// deprecated alias for F19
pub const FCITX_KEY_L9: u32 = 0xffd0;

pub const FCITX_KEY_F20: u32 = 0xffd1;

/// deprecated alias for F20
pub const FCITX_KEY_L10: u32 = 0xffd1;

pub const FCITX_KEY_F21: u32 = 0xffd2;

/// deprecated alias for F21
pub const FCITX_KEY_R1: u32 = 0xffd2;

pub const FCITX_KEY_F22: u32 = 0xffd3;

/// deprecated alias for F22
pub const FCITX_KEY_R2: u32 = 0xffd3;

pub const FCITX_KEY_F23: u32 = 0xffd4;

/// deprecated alias for F23
pub const FCITX_KEY_R3: u32 = 0xffd4;

pub const FCITX_KEY_F24: u32 = 0xffd5;

/// deprecated alias for F24
pub const FCITX_KEY_R4: u32 = 0xffd5;

pub const FCITX_KEY_F25: u32 = 0xffd6;

/// deprecated alias for F25
pub const FCITX_KEY_R5: u32 = 0xffd6;

pub const FCITX_KEY_F26: u32 = 0xffd7;

/// deprecated alias for F26
pub const FCITX_KEY_R6: u32 = 0xffd7;

pub const FCITX_KEY_F27: u32 = 0xffd8;

/// deprecated alias for F27
pub const FCITX_KEY_R7: u32 = 0xffd8;

pub const FCITX_KEY_F28: u32 = 0xffd9;

/// deprecated alias for F28
pub const FCITX_KEY_R8: u32 = 0xffd9;

pub const FCITX_KEY_F29: u32 = 0xffda;

/// deprecated alias for F29
pub const FCITX_KEY_R9: u32 = 0xffda;

pub const FCITX_KEY_F30: u32 = 0xffdb;

/// deprecated alias for F30
pub const FCITX_KEY_R10: u32 = 0xffdb;

pub const FCITX_KEY_F31: u32 = 0xffdc;

/// deprecated alias for F31
pub const FCITX_KEY_R11: u32 = 0xffdc;

pub const FCITX_KEY_F32: u32 = 0xffdd;

/// deprecated alias for F32
pub const FCITX_KEY_R12: u32 = 0xffdd;

pub const FCITX_KEY_F33: u32 = 0xffde;

/// deprecated alias for F33
pub const FCITX_KEY_R13: u32 = 0xffde;

pub const FCITX_KEY_F34: u32 = 0xffdf;

/// deprecated alias for F34
pub const FCITX_KEY_R14: u32 = 0xffdf;

pub const FCITX_KEY_F35: u32 = 0xffe0;

/// deprecated alias for F35
pub const FCITX_KEY_R15: u32 = 0xffe0;

/// Left shift
pub const FCITX_KEY_Shift_L: u32 = 0xffe1;

/// Right shift
pub const FCITX_KEY_Shift_R: u32 = 0xffe2;

/// Left control
pub const FCITX_KEY_Control_L: u32 = 0xffe3;

/// Right control
pub const FCITX_KEY_Control_R: u32 = 0xffe4;

/// Caps lock
pub const FCITX_KEY_Caps_Lock: u32 = 0xffe5;

/// Shift lock
pub const FCITX_KEY_Shift_Lock: u32 = 0xffe6;

/// Left meta
pub const FCITX_KEY_Meta_L: u32 = 0xffe7;

/// Right meta
pub const FCITX_KEY_Meta_R: u32 = 0xffe8;

/// Left alt
pub const FCITX_KEY_Alt_L: u32 = 0xffe9;

/// Right alt
pub const FCITX_KEY_Alt_R: u32 = 0xffea;

/// Left super
pub const FCITX_KEY_Super_L: u32 = 0xffeb;

/// Right super
pub const FCITX_KEY_Super_R: u32 = 0xffec;

/// Left hyper
pub const FCITX_KEY_Hyper_L: u32 = 0xffed;

/// Right hyper
pub const FCITX_KEY_Hyper_R: u32 = 0xffee;

pub const FCITX_KEY_ISO_Lock: u32 = 0xfe01;

pub const FCITX_KEY_ISO_Level2_Latch: u32 = 0xfe02;

pub const FCITX_KEY_ISO_Level3_Shift: u32 = 0xfe03;

pub const FCITX_KEY_ISO_Level3_Latch: u32 = 0xfe04;

pub const FCITX_KEY_ISO_Level3_Lock: u32 = 0xfe05;

pub const FCITX_KEY_ISO_Level5_Shift: u32 = 0xfe11;

pub const FCITX_KEY_ISO_Level5_Latch: u32 = 0xfe12;

pub const FCITX_KEY_ISO_Level5_Lock: u32 = 0xfe13;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_ISO_Group_Shift: u32 = 0xff7e;

pub const FCITX_KEY_ISO_Group_Latch: u32 = 0xfe06;

pub const FCITX_KEY_ISO_Group_Lock: u32 = 0xfe07;

pub const FCITX_KEY_ISO_Next_Group: u32 = 0xfe08;

pub const FCITX_KEY_ISO_Next_Group_Lock: u32 = 0xfe09;

pub const FCITX_KEY_ISO_Prev_Group: u32 = 0xfe0a;

pub const FCITX_KEY_ISO_Prev_Group_Lock: u32 = 0xfe0b;

pub const FCITX_KEY_ISO_First_Group: u32 = 0xfe0c;

pub const FCITX_KEY_ISO_First_Group_Lock: u32 = 0xfe0d;

pub const FCITX_KEY_ISO_Last_Group: u32 = 0xfe0e;

pub const FCITX_KEY_ISO_Last_Group_Lock: u32 = 0xfe0f;

pub const FCITX_KEY_ISO_Left_Tab: u32 = 0xfe20;

pub const FCITX_KEY_ISO_Move_Line_Up: u32 = 0xfe21;

pub const FCITX_KEY_ISO_Move_Line_Down: u32 = 0xfe22;

pub const FCITX_KEY_ISO_Partial_Line_Up: u32 = 0xfe23;

pub const FCITX_KEY_ISO_Partial_Line_Down: u32 = 0xfe24;

pub const FCITX_KEY_ISO_Partial_Space_Left: u32 = 0xfe25;

pub const FCITX_KEY_ISO_Partial_Space_Right: u32 = 0xfe26;

pub const FCITX_KEY_ISO_Set_Margin_Left: u32 = 0xfe27;

pub const FCITX_KEY_ISO_Set_Margin_Right: u32 = 0xfe28;

pub const FCITX_KEY_ISO_Release_Margin_Left: u32 = 0xfe29;

pub const FCITX_KEY_ISO_Release_Margin_Right: u32 = 0xfe2a;

pub const FCITX_KEY_ISO_Release_Both_Margins: u32 = 0xfe2b;

pub const FCITX_KEY_ISO_Fast_Cursor_Left: u32 = 0xfe2c;

pub const FCITX_KEY_ISO_Fast_Cursor_Right: u32 = 0xfe2d;

pub const FCITX_KEY_ISO_Fast_Cursor_Up: u32 = 0xfe2e;

pub const FCITX_KEY_ISO_Fast_Cursor_Down: u32 = 0xfe2f;

pub const FCITX_KEY_ISO_Continuous_Underline: u32 = 0xfe30;

pub const FCITX_KEY_ISO_Discontinuous_Underline: u32 = 0xfe31;

pub const FCITX_KEY_ISO_Emphasize: u32 = 0xfe32;

pub const FCITX_KEY_ISO_Center_Object: u32 = 0xfe33;

pub const FCITX_KEY_ISO_Enter: u32 = 0xfe34;

pub const FCITX_KEY_dead_grave: u32 = 0xfe50;

pub const FCITX_KEY_dead_acute: u32 = 0xfe51;

pub const FCITX_KEY_dead_circumflex: u32 = 0xfe52;

pub const FCITX_KEY_dead_tilde: u32 = 0xfe53;

/// non-deprecated alias for dead_tilde
pub const FCITX_KEY_dead_perispomeni: u32 = 0xfe53;

pub const FCITX_KEY_dead_macron: u32 = 0xfe54;

pub const FCITX_KEY_dead_breve: u32 = 0xfe55;

pub const FCITX_KEY_dead_abovedot: u32 = 0xfe56;

pub const FCITX_KEY_dead_diaeresis: u32 = 0xfe57;

pub const FCITX_KEY_dead_abovering: u32 = 0xfe58;

pub const FCITX_KEY_dead_doubleacute: u32 = 0xfe59;

pub const FCITX_KEY_dead_caron: u32 = 0xfe5a;

pub const FCITX_KEY_dead_cedilla: u32 = 0xfe5b;

pub const FCITX_KEY_dead_ogonek: u32 = 0xfe5c;

pub const FCITX_KEY_dead_iota: u32 = 0xfe5d;

pub const FCITX_KEY_dead_voiced_sound: u32 = 0xfe5e;

pub const FCITX_KEY_dead_semivoiced_sound: u32 = 0xfe5f;

pub const FCITX_KEY_dead_belowdot: u32 = 0xfe60;

pub const FCITX_KEY_dead_hook: u32 = 0xfe61;

pub const FCITX_KEY_dead_horn: u32 = 0xfe62;

pub const FCITX_KEY_dead_stroke: u32 = 0xfe63;

pub const FCITX_KEY_dead_abovecomma: u32 = 0xfe64;

/// non-deprecated alias for dead_abovecomma
pub const FCITX_KEY_dead_psili: u32 = 0xfe64;

pub const FCITX_KEY_dead_abovereversedcomma: u32 = 0xfe65;

/// non-deprecated alias for dead_abovereversedcomma
pub const FCITX_KEY_dead_dasia: u32 = 0xfe65;

pub const FCITX_KEY_dead_doublegrave: u32 = 0xfe66;

pub const FCITX_KEY_dead_belowring: u32 = 0xfe67;

pub const FCITX_KEY_dead_belowmacron: u32 = 0xfe68;

pub const FCITX_KEY_dead_belowcircumflex: u32 = 0xfe69;

pub const FCITX_KEY_dead_belowtilde: u32 = 0xfe6a;

pub const FCITX_KEY_dead_belowbreve: u32 = 0xfe6b;

pub const FCITX_KEY_dead_belowdiaeresis: u32 = 0xfe6c;

pub const FCITX_KEY_dead_invertedbreve: u32 = 0xfe6d;

pub const FCITX_KEY_dead_belowcomma: u32 = 0xfe6e;

pub const FCITX_KEY_dead_currency: u32 = 0xfe6f;

pub const FCITX_KEY_dead_lowline: u32 = 0xfe90;

pub const FCITX_KEY_dead_aboveverticalline: u32 = 0xfe91;

pub const FCITX_KEY_dead_belowverticalline: u32 = 0xfe92;

pub const FCITX_KEY_dead_longsolidusoverlay: u32 = 0xfe93;

pub const FCITX_KEY_dead_a: u32 = 0xfe80;

pub const FCITX_KEY_dead_A: u32 = 0xfe81;

pub const FCITX_KEY_dead_e: u32 = 0xfe82;

pub const FCITX_KEY_dead_E: u32 = 0xfe83;

pub const FCITX_KEY_dead_i: u32 = 0xfe84;

pub const FCITX_KEY_dead_I: u32 = 0xfe85;

pub const FCITX_KEY_dead_o: u32 = 0xfe86;

pub const FCITX_KEY_dead_O: u32 = 0xfe87;

pub const FCITX_KEY_dead_u: u32 = 0xfe88;

pub const FCITX_KEY_dead_U: u32 = 0xfe89;

/// deprecated alias for dead_schwa
pub const FCITX_KEY_dead_small_schwa: u32 = 0xfe8a;

pub const FCITX_KEY_dead_schwa: u32 = 0xfe8a;

/// deprecated alias for dead_SCHWA
pub const FCITX_KEY_dead_capital_schwa: u32 = 0xfe8b;

pub const FCITX_KEY_dead_SCHWA: u32 = 0xfe8b;

pub const FCITX_KEY_dead_greek: u32 = 0xfe8c;

pub const FCITX_KEY_dead_hamza: u32 = 0xfe8d;

pub const FCITX_KEY_First_Virtual_Screen: u32 = 0xfed0;

pub const FCITX_KEY_Prev_Virtual_Screen: u32 = 0xfed1;

pub const FCITX_KEY_Next_Virtual_Screen: u32 = 0xfed2;

pub const FCITX_KEY_Last_Virtual_Screen: u32 = 0xfed4;

pub const FCITX_KEY_Terminate_Server: u32 = 0xfed5;

pub const FCITX_KEY_AccessX_Enable: u32 = 0xfe70;

pub const FCITX_KEY_AccessX_Feedback_Enable: u32 = 0xfe71;

pub const FCITX_KEY_RepeatKeys_Enable: u32 = 0xfe72;

pub const FCITX_KEY_SlowKeys_Enable: u32 = 0xfe73;

pub const FCITX_KEY_BounceKeys_Enable: u32 = 0xfe74;

pub const FCITX_KEY_StickyKeys_Enable: u32 = 0xfe75;

pub const FCITX_KEY_MouseKeys_Enable: u32 = 0xfe76;

pub const FCITX_KEY_MouseKeys_Accel_Enable: u32 = 0xfe77;

pub const FCITX_KEY_Overlay1_Enable: u32 = 0xfe78;

pub const FCITX_KEY_Overlay2_Enable: u32 = 0xfe79;

pub const FCITX_KEY_AudibleBell_Enable: u32 = 0xfe7a;

pub const FCITX_KEY_Pointer_Left: u32 = 0xfee0;

pub const FCITX_KEY_Pointer_Right: u32 = 0xfee1;

pub const FCITX_KEY_Pointer_Up: u32 = 0xfee2;

pub const FCITX_KEY_Pointer_Down: u32 = 0xfee3;

pub const FCITX_KEY_Pointer_UpLeft: u32 = 0xfee4;

pub const FCITX_KEY_Pointer_UpRight: u32 = 0xfee5;

pub const FCITX_KEY_Pointer_DownLeft: u32 = 0xfee6;

pub const FCITX_KEY_Pointer_DownRight: u32 = 0xfee7;

pub const FCITX_KEY_Pointer_Button_Dflt: u32 = 0xfee8;

pub const FCITX_KEY_Pointer_Button1: u32 = 0xfee9;

pub const FCITX_KEY_Pointer_Button2: u32 = 0xfeea;

pub const FCITX_KEY_Pointer_Button3: u32 = 0xfeeb;

pub const FCITX_KEY_Pointer_Button4: u32 = 0xfeec;

pub const FCITX_KEY_Pointer_Button5: u32 = 0xfeed;

pub const FCITX_KEY_Pointer_DblClick_Dflt: u32 = 0xfeee;

pub const FCITX_KEY_Pointer_DblClick1: u32 = 0xfeef;

pub const FCITX_KEY_Pointer_DblClick2: u32 = 0xfef0;

pub const FCITX_KEY_Pointer_DblClick3: u32 = 0xfef1;

pub const FCITX_KEY_Pointer_DblClick4: u32 = 0xfef2;

pub const FCITX_KEY_Pointer_DblClick5: u32 = 0xfef3;

pub const FCITX_KEY_Pointer_Drag_Dflt: u32 = 0xfef4;

pub const FCITX_KEY_Pointer_Drag1: u32 = 0xfef5;

pub const FCITX_KEY_Pointer_Drag2: u32 = 0xfef6;

pub const FCITX_KEY_Pointer_Drag3: u32 = 0xfef7;

pub const FCITX_KEY_Pointer_Drag4: u32 = 0xfef8;

pub const FCITX_KEY_Pointer_Drag5: u32 = 0xfefd;

pub const FCITX_KEY_Pointer_EnableKeys: u32 = 0xfef9;

pub const FCITX_KEY_Pointer_Accelerate: u32 = 0xfefa;

pub const FCITX_KEY_Pointer_DfltBtnNext: u32 = 0xfefb;

pub const FCITX_KEY_Pointer_DfltBtnPrev: u32 = 0xfefc;

pub const FCITX_KEY_ch: u32 = 0xfea0;

pub const FCITX_KEY_Ch: u32 = 0xfea1;

pub const FCITX_KEY_CH: u32 = 0xfea2;

pub const FCITX_KEY_c_h: u32 = 0xfea3;

pub const FCITX_KEY_C_h: u32 = 0xfea4;

pub const FCITX_KEY_C_H: u32 = 0xfea5;

pub const FCITX_KEY_3270_Duplicate: u32 = 0xfd01;

pub const FCITX_KEY_3270_FieldMark: u32 = 0xfd02;

pub const FCITX_KEY_3270_Right2: u32 = 0xfd03;

pub const FCITX_KEY_3270_Left2: u32 = 0xfd04;

pub const FCITX_KEY_3270_BackTab: u32 = 0xfd05;

pub const FCITX_KEY_3270_EraseEOF: u32 = 0xfd06;

pub const FCITX_KEY_3270_EraseInput: u32 = 0xfd07;

pub const FCITX_KEY_3270_Reset: u32 = 0xfd08;

pub const FCITX_KEY_3270_Quit: u32 = 0xfd09;

pub const FCITX_KEY_3270_PA1: u32 = 0xfd0a;

pub const FCITX_KEY_3270_PA2: u32 = 0xfd0b;

pub const FCITX_KEY_3270_PA3: u32 = 0xfd0c;

pub const FCITX_KEY_3270_Test: u32 = 0xfd0d;

pub const FCITX_KEY_3270_Attn: u32 = 0xfd0e;

pub const FCITX_KEY_3270_CursorBlink: u32 = 0xfd0f;

pub const FCITX_KEY_3270_AltCursor: u32 = 0xfd10;

pub const FCITX_KEY_3270_KeyClick: u32 = 0xfd11;

pub const FCITX_KEY_3270_Jump: u32 = 0xfd12;

pub const FCITX_KEY_3270_Ident: u32 = 0xfd13;

pub const FCITX_KEY_3270_Rule: u32 = 0xfd14;

pub const FCITX_KEY_3270_Copy: u32 = 0xfd15;

pub const FCITX_KEY_3270_Play: u32 = 0xfd16;

pub const FCITX_KEY_3270_Setup: u32 = 0xfd17;

pub const FCITX_KEY_3270_Record: u32 = 0xfd18;

pub const FCITX_KEY_3270_ChangeScreen: u32 = 0xfd19;

pub const FCITX_KEY_3270_DeleteWord: u32 = 0xfd1a;

pub const FCITX_KEY_3270_ExSelect: u32 = 0xfd1b;

pub const FCITX_KEY_3270_CursorSelect: u32 = 0xfd1c;

pub const FCITX_KEY_3270_PrintScreen: u32 = 0xfd1d;

pub const FCITX_KEY_3270_Enter: u32 = 0xfd1e;

/// U+0020 SPACE
pub const FCITX_KEY_space: u32 = 0x0020;

/// U+0021 EXCLAMATION MARK
pub const FCITX_KEY_exclam: u32 = 0x0021;

/// U+0022 QUOTATION MARK
pub const FCITX_KEY_quotedbl: u32 = 0x0022;

/// U+0023 NUMBER SIGN
pub const FCITX_KEY_numbersign: u32 = 0x0023;

/// U+0024 DOLLAR SIGN
pub const FCITX_KEY_dollar: u32 = 0x0024;

/// U+0025 PERCENT SIGN
pub const FCITX_KEY_percent: u32 = 0x0025;

/// U+0026 AMPERSAND
pub const FCITX_KEY_ampersand: u32 = 0x0026;

/// U+0027 APOSTROPHE
pub const FCITX_KEY_apostrophe: u32 = 0x0027;

/// deprecated
pub const FCITX_KEY_quoteright: u32 = 0x0027;

/// U+0028 LEFT PARENTHESIS
pub const FCITX_KEY_parenleft: u32 = 0x0028;

/// U+0029 RIGHT PARENTHESIS
pub const FCITX_KEY_parenright: u32 = 0x0029;

/// U+002A ASTERISK
pub const FCITX_KEY_asterisk: u32 = 0x002a;

/// U+002B PLUS SIGN
pub const FCITX_KEY_plus: u32 = 0x002b;

/// U+002C COMMA
pub const FCITX_KEY_comma: u32 = 0x002c;

/// U+002D HYPHEN-MINUS
pub const FCITX_KEY_minus: u32 = 0x002d;

/// U+002E FULL STOP
pub const FCITX_KEY_period: u32 = 0x002e;

/// U+002F SOLIDUS
pub const FCITX_KEY_slash: u32 = 0x002f;

/// U+0030 DIGIT ZERO
pub const FCITX_KEY_0: u32 = 0x0030;

/// U+0031 DIGIT ONE
pub const FCITX_KEY_1: u32 = 0x0031;

/// U+0032 DIGIT TWO
pub const FCITX_KEY_2: u32 = 0x0032;

/// U+0033 DIGIT THREE
pub const FCITX_KEY_3: u32 = 0x0033;

/// U+0034 DIGIT FOUR
pub const FCITX_KEY_4: u32 = 0x0034;

/// U+0035 DIGIT FIVE
pub const FCITX_KEY_5: u32 = 0x0035;

/// U+0036 DIGIT SIX
pub const FCITX_KEY_6: u32 = 0x0036;

/// U+0037 DIGIT SEVEN
pub const FCITX_KEY_7: u32 = 0x0037;

/// U+0038 DIGIT EIGHT
pub const FCITX_KEY_8: u32 = 0x0038;

/// U+0039 DIGIT NINE
pub const FCITX_KEY_9: u32 = 0x0039;

/// U+003A COLON
pub const FCITX_KEY_colon: u32 = 0x003a;

/// U+003B SEMICOLON
pub const FCITX_KEY_semicolon: u32 = 0x003b;

/// U+003C LESS-THAN SIGN
pub const FCITX_KEY_less: u32 = 0x003c;

/// U+003D EQUALS SIGN
pub const FCITX_KEY_equal: u32 = 0x003d;

/// U+003E GREATER-THAN SIGN
pub const FCITX_KEY_greater: u32 = 0x003e;

/// U+003F QUESTION MARK
pub const FCITX_KEY_question: u32 = 0x003f;

/// U+0040 COMMERCIAL AT
pub const FCITX_KEY_at: u32 = 0x0040;

/// U+0041 LATIN CAPITAL LETTER A
pub const FCITX_KEY_A: u32 = 0x0041;

/// U+0042 LATIN CAPITAL LETTER B
pub const FCITX_KEY_B: u32 = 0x0042;

/// U+0043 LATIN CAPITAL LETTER C
pub const FCITX_KEY_C: u32 = 0x0043;

/// U+0044 LATIN CAPITAL LETTER D
pub const FCITX_KEY_D: u32 = 0x0044;

/// U+0045 LATIN CAPITAL LETTER E
pub const FCITX_KEY_E: u32 = 0x0045;

/// U+0046 LATIN CAPITAL LETTER F
pub const FCITX_KEY_F: u32 = 0x0046;

/// U+0047 LATIN CAPITAL LETTER G
pub const FCITX_KEY_G: u32 = 0x0047;

/// U+0048 LATIN CAPITAL LETTER H
pub const FCITX_KEY_H: u32 = 0x0048;

/// U+0049 LATIN CAPITAL LETTER I
pub const FCITX_KEY_I: u32 = 0x0049;

/// U+004A LATIN CAPITAL LETTER J
pub const FCITX_KEY_J: u32 = 0x004a;

/// U+004B LATIN CAPITAL LETTER K
pub const FCITX_KEY_K: u32 = 0x004b;

/// U+004C LATIN CAPITAL LETTER L
pub const FCITX_KEY_L: u32 = 0x004c;

/// U+004D LATIN CAPITAL LETTER M
pub const FCITX_KEY_M: u32 = 0x004d;

/// U+004E LATIN CAPITAL LETTER N
pub const FCITX_KEY_N: u32 = 0x004e;

/// U+004F LATIN CAPITAL LETTER O
pub const FCITX_KEY_O: u32 = 0x004f;

/// U+0050 LATIN CAPITAL LETTER P
pub const FCITX_KEY_P: u32 = 0x0050;

/// U+0051 LATIN CAPITAL LETTER Q
pub const FCITX_KEY_Q: u32 = 0x0051;

/// U+0052 LATIN CAPITAL LETTER R
pub const FCITX_KEY_R: u32 = 0x0052;

/// U+0053 LATIN CAPITAL LETTER S
pub const FCITX_KEY_S: u32 = 0x0053;

/// U+0054 LATIN CAPITAL LETTER T
pub const FCITX_KEY_T: u32 = 0x0054;

/// U+0055 LATIN CAPITAL LETTER U
pub const FCITX_KEY_U: u32 = 0x0055;

/// U+0056 LATIN CAPITAL LETTER V
pub const FCITX_KEY_V: u32 = 0x0056;

/// U+0057 LATIN CAPITAL LETTER W
pub const FCITX_KEY_W: u32 = 0x0057;

/// U+0058 LATIN CAPITAL LETTER X
pub const FCITX_KEY_X: u32 = 0x0058;

/// U+0059 LATIN CAPITAL LETTER Y
pub const FCITX_KEY_Y: u32 = 0x0059;

/// U+005A LATIN CAPITAL LETTER Z
pub const FCITX_KEY_Z: u32 = 0x005a;

/// U+005B LEFT SQUARE BRACKET
pub const FCITX_KEY_bracketleft: u32 = 0x005b;

/// U+005C REVERSE SOLIDUS
pub const FCITX_KEY_backslash: u32 = 0x005c;

/// U+005D RIGHT SQUARE BRACKET
pub const FCITX_KEY_bracketright: u32 = 0x005d;

/// U+005E CIRCUMFLEX ACCENT
pub const FCITX_KEY_asciicircum: u32 = 0x005e;

/// U+005F LOW LINE
pub const FCITX_KEY_underscore: u32 = 0x005f;

/// U+0060 GRAVE ACCENT
pub const FCITX_KEY_grave: u32 = 0x0060;

/// deprecated
pub const FCITX_KEY_quoteleft: u32 = 0x0060;

/// U+0061 LATIN SMALL LETTER A
pub const FCITX_KEY_a: u32 = 0x0061;

/// U+0062 LATIN SMALL LETTER B
pub const FCITX_KEY_b: u32 = 0x0062;

/// U+0063 LATIN SMALL LETTER C
pub const FCITX_KEY_c: u32 = 0x0063;

/// U+0064 LATIN SMALL LETTER D
pub const FCITX_KEY_d: u32 = 0x0064;

/// U+0065 LATIN SMALL LETTER E
pub const FCITX_KEY_e: u32 = 0x0065;

/// U+0066 LATIN SMALL LETTER F
pub const FCITX_KEY_f: u32 = 0x0066;

/// U+0067 LATIN SMALL LETTER G
pub const FCITX_KEY_g: u32 = 0x0067;

/// U+0068 LATIN SMALL LETTER H
pub const FCITX_KEY_h: u32 = 0x0068;

/// U+0069 LATIN SMALL LETTER I
pub const FCITX_KEY_i: u32 = 0x0069;

/// U+006A LATIN SMALL LETTER J
pub const FCITX_KEY_j: u32 = 0x006a;

/// U+006B LATIN SMALL LETTER K
pub const FCITX_KEY_k: u32 = 0x006b;

/// U+006C LATIN SMALL LETTER L
pub const FCITX_KEY_l: u32 = 0x006c;

/// U+006D LATIN SMALL LETTER M
pub const FCITX_KEY_m: u32 = 0x006d;

/// U+006E LATIN SMALL LETTER N
pub const FCITX_KEY_n: u32 = 0x006e;

/// U+006F LATIN SMALL LETTER O
pub const FCITX_KEY_o: u32 = 0x006f;

/// U+0070 LATIN SMALL LETTER P
pub const FCITX_KEY_p: u32 = 0x0070;

/// U+0071 LATIN SMALL LETTER Q
pub const FCITX_KEY_q: u32 = 0x0071;

/// U+0072 LATIN SMALL LETTER R
pub const FCITX_KEY_r: u32 = 0x0072;

/// U+0073 LATIN SMALL LETTER S
pub const FCITX_KEY_s: u32 = 0x0073;

/// U+0074 LATIN SMALL LETTER T
pub const FCITX_KEY_t: u32 = 0x0074;

/// U+0075 LATIN SMALL LETTER U
pub const FCITX_KEY_u: u32 = 0x0075;

/// U+0076 LATIN SMALL LETTER V
pub const FCITX_KEY_v: u32 = 0x0076;

/// U+0077 LATIN SMALL LETTER W
pub const FCITX_KEY_w: u32 = 0x0077;

/// U+0078 LATIN SMALL LETTER X
pub const FCITX_KEY_x: u32 = 0x0078;

/// U+0079 LATIN SMALL LETTER Y
pub const FCITX_KEY_y: u32 = 0x0079;

/// U+007A LATIN SMALL LETTER Z
pub const FCITX_KEY_z: u32 = 0x007a;

/// U+007B LEFT CURLY BRACKET
pub const FCITX_KEY_braceleft: u32 = 0x007b;

/// U+007C VERTICAL LINE
pub const FCITX_KEY_bar: u32 = 0x007c;

/// U+007D RIGHT CURLY BRACKET
pub const FCITX_KEY_braceright: u32 = 0x007d;

/// U+007E TILDE
pub const FCITX_KEY_asciitilde: u32 = 0x007e;

/// U+00A0 NO-BREAK SPACE
pub const FCITX_KEY_nobreakspace: u32 = 0x00a0;

/// U+00A1 INVERTED EXCLAMATION MARK
pub const FCITX_KEY_exclamdown: u32 = 0x00a1;

/// U+00A2 CENT SIGN
pub const FCITX_KEY_cent: u32 = 0x00a2;

/// U+00A3 POUND SIGN
pub const FCITX_KEY_sterling: u32 = 0x00a3;

/// U+00A4 CURRENCY SIGN
pub const FCITX_KEY_currency: u32 = 0x00a4;

/// U+00A5 YEN SIGN
pub const FCITX_KEY_yen: u32 = 0x00a5;

/// U+00A6 BROKEN BAR
pub const FCITX_KEY_brokenbar: u32 = 0x00a6;

/// U+00A7 SECTION SIGN
pub const FCITX_KEY_section: u32 = 0x00a7;

/// U+00A8 DIAERESIS
pub const FCITX_KEY_diaeresis: u32 = 0x00a8;

/// U+00A9 COPYRIGHT SIGN
pub const FCITX_KEY_copyright: u32 = 0x00a9;

/// U+00AA FEMININE ORDINAL INDICATOR
pub const FCITX_KEY_ordfeminine: u32 = 0x00aa;

/// deprecated alias for guillemetleft (misspelling)
pub const FCITX_KEY_guillemotleft: u32 = 0x00ab;

/// U+00AB LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
pub const FCITX_KEY_guillemetleft: u32 = 0x00ab;

/// U+00AC NOT SIGN
pub const FCITX_KEY_notsign: u32 = 0x00ac;

/// U+00AD SOFT HYPHEN
pub const FCITX_KEY_hyphen: u32 = 0x00ad;

/// U+00AE REGISTERED SIGN
pub const FCITX_KEY_registered: u32 = 0x00ae;

/// U+00AF MACRON
pub const FCITX_KEY_macron: u32 = 0x00af;

/// U+00B0 DEGREE SIGN
pub const FCITX_KEY_degree: u32 = 0x00b0;

/// U+00B1 PLUS-MINUS SIGN
pub const FCITX_KEY_plusminus: u32 = 0x00b1;

/// U+00B2 SUPERSCRIPT TWO
pub const FCITX_KEY_twosuperior: u32 = 0x00b2;

/// U+00B3 SUPERSCRIPT THREE
pub const FCITX_KEY_threesuperior: u32 = 0x00b3;

/// U+00B4 ACUTE ACCENT
pub const FCITX_KEY_acute: u32 = 0x00b4;

/// U+00B5 MICRO SIGN
pub const FCITX_KEY_mu: u32 = 0x00b5;

/// U+00B6 PILCROW SIGN
pub const FCITX_KEY_paragraph: u32 = 0x00b6;

/// U+00B7 MIDDLE DOT
pub const FCITX_KEY_periodcentered: u32 = 0x00b7;

/// U+00B8 CEDILLA
pub const FCITX_KEY_cedilla: u32 = 0x00b8;

/// U+00B9 SUPERSCRIPT ONE
pub const FCITX_KEY_onesuperior: u32 = 0x00b9;

/// deprecated alias for ordmasculine (inconsistent name)
pub const FCITX_KEY_masculine: u32 = 0x00ba;

/// U+00BA MASCULINE ORDINAL INDICATOR
pub const FCITX_KEY_ordmasculine: u32 = 0x00ba;

/// deprecated alias for guillemetright (misspelling)
pub const FCITX_KEY_guillemotright: u32 = 0x00bb;

/// U+00BB RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
pub const FCITX_KEY_guillemetright: u32 = 0x00bb;

/// U+00BC VULGAR FRACTION ONE QUARTER
pub const FCITX_KEY_onequarter: u32 = 0x00bc;

/// U+00BD VULGAR FRACTION ONE HALF
pub const FCITX_KEY_onehalf: u32 = 0x00bd;

/// U+00BE VULGAR FRACTION THREE QUARTERS
pub const FCITX_KEY_threequarters: u32 = 0x00be;

/// U+00BF INVERTED QUESTION MARK
pub const FCITX_KEY_questiondown: u32 = 0x00bf;

/// U+00C0 LATIN CAPITAL LETTER A WITH GRAVE
pub const FCITX_KEY_Agrave: u32 = 0x00c0;

/// U+00C1 LATIN CAPITAL LETTER A WITH ACUTE
pub const FCITX_KEY_Aacute: u32 = 0x00c1;

/// U+00C2 LATIN CAPITAL LETTER A WITH CIRCUMFLEX
pub const FCITX_KEY_Acircumflex: u32 = 0x00c2;

/// U+00C3 LATIN CAPITAL LETTER A WITH TILDE
pub const FCITX_KEY_Atilde: u32 = 0x00c3;

/// U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
pub const FCITX_KEY_Adiaeresis: u32 = 0x00c4;

/// U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE
pub const FCITX_KEY_Aring: u32 = 0x00c5;

/// U+00C6 LATIN CAPITAL LETTER AE
pub const FCITX_KEY_AE: u32 = 0x00c6;

/// U+00C7 LATIN CAPITAL LETTER C WITH CEDILLA
pub const FCITX_KEY_Ccedilla: u32 = 0x00c7;

/// U+00C8 LATIN CAPITAL LETTER E WITH GRAVE
pub const FCITX_KEY_Egrave: u32 = 0x00c8;

/// U+00C9 LATIN CAPITAL LETTER E WITH ACUTE
pub const FCITX_KEY_Eacute: u32 = 0x00c9;

/// U+00CA LATIN CAPITAL LETTER E WITH CIRCUMFLEX
pub const FCITX_KEY_Ecircumflex: u32 = 0x00ca;

/// U+00CB LATIN CAPITAL LETTER E WITH DIAERESIS
pub const FCITX_KEY_Ediaeresis: u32 = 0x00cb;

/// U+00CC LATIN CAPITAL LETTER I WITH GRAVE
pub const FCITX_KEY_Igrave: u32 = 0x00cc;

/// U+00CD LATIN CAPITAL LETTER I WITH ACUTE
pub const FCITX_KEY_Iacute: u32 = 0x00cd;

/// U+00CE LATIN CAPITAL LETTER I WITH CIRCUMFLEX
pub const FCITX_KEY_Icircumflex: u32 = 0x00ce;

/// U+00CF LATIN CAPITAL LETTER I WITH DIAERESIS
pub const FCITX_KEY_Idiaeresis: u32 = 0x00cf;

/// U+00D0 LATIN CAPITAL LETTER ETH
pub const FCITX_KEY_ETH: u32 = 0x00d0;

/// deprecated
pub const FCITX_KEY_Eth: u32 = 0x00d0;

/// U+00D1 LATIN CAPITAL LETTER N WITH TILDE
pub const FCITX_KEY_Ntilde: u32 = 0x00d1;

/// U+00D2 LATIN CAPITAL LETTER O WITH GRAVE
pub const FCITX_KEY_Ograve: u32 = 0x00d2;

/// U+00D3 LATIN CAPITAL LETTER O WITH ACUTE
pub const FCITX_KEY_Oacute: u32 = 0x00d3;

/// U+00D4 LATIN CAPITAL LETTER O WITH CIRCUMFLEX
pub const FCITX_KEY_Ocircumflex: u32 = 0x00d4;

/// U+00D5 LATIN CAPITAL LETTER O WITH TILDE
pub const FCITX_KEY_Otilde: u32 = 0x00d5;

/// U+00D6 LATIN CAPITAL LETTER O WITH DIAERESIS
pub const FCITX_KEY_Odiaeresis: u32 = 0x00d6;

/// U+00D7 MULTIPLICATION SIGN
pub const FCITX_KEY_multiply: u32 = 0x00d7;

/// U+00D8 LATIN CAPITAL LETTER O WITH STROKE
pub const FCITX_KEY_Oslash: u32 = 0x00d8;

/// deprecated alias for Oslash
pub const FCITX_KEY_Ooblique: u32 = 0x00d8;

/// U+00D9 LATIN CAPITAL LETTER U WITH GRAVE
pub const FCITX_KEY_Ugrave: u32 = 0x00d9;

/// U+00DA LATIN CAPITAL LETTER U WITH ACUTE
pub const FCITX_KEY_Uacute: u32 = 0x00da;

/// U+00DB LATIN CAPITAL LETTER U WITH CIRCUMFLEX
pub const FCITX_KEY_Ucircumflex: u32 = 0x00db;

/// U+00DC LATIN CAPITAL LETTER U WITH DIAERESIS
pub const FCITX_KEY_Udiaeresis: u32 = 0x00dc;

/// U+00DD LATIN CAPITAL LETTER Y WITH ACUTE
pub const FCITX_KEY_Yacute: u32 = 0x00dd;

/// U+00DE LATIN CAPITAL LETTER THORN
pub const FCITX_KEY_THORN: u32 = 0x00de;

/// deprecated
pub const FCITX_KEY_Thorn: u32 = 0x00de;

/// U+00DF LATIN SMALL LETTER SHARP S
pub const FCITX_KEY_ssharp: u32 = 0x00df;

/// U+00E0 LATIN SMALL LETTER A WITH GRAVE
pub const FCITX_KEY_agrave: u32 = 0x00e0;

/// U+00E1 LATIN SMALL LETTER A WITH ACUTE
pub const FCITX_KEY_aacute: u32 = 0x00e1;

/// U+00E2 LATIN SMALL LETTER A WITH CIRCUMFLEX
pub const FCITX_KEY_acircumflex: u32 = 0x00e2;

/// U+00E3 LATIN SMALL LETTER A WITH TILDE
pub const FCITX_KEY_atilde: u32 = 0x00e3;

/// U+00E4 LATIN SMALL LETTER A WITH DIAERESIS
pub const FCITX_KEY_adiaeresis: u32 = 0x00e4;

/// U+00E5 LATIN SMALL LETTER A WITH RING ABOVE
pub const FCITX_KEY_aring: u32 = 0x00e5;

/// U+00E6 LATIN SMALL LETTER AE
pub const FCITX_KEY_ae: u32 = 0x00e6;

/// U+00E7 LATIN SMALL LETTER C WITH CEDILLA
pub const FCITX_KEY_ccedilla: u32 = 0x00e7;

/// U+00E8 LATIN SMALL LETTER E WITH GRAVE
pub const FCITX_KEY_egrave: u32 = 0x00e8;

/// U+00E9 LATIN SMALL LETTER E WITH ACUTE
pub const FCITX_KEY_eacute: u32 = 0x00e9;

/// U+00EA LATIN SMALL LETTER E WITH CIRCUMFLEX
pub const FCITX_KEY_ecircumflex: u32 = 0x00ea;

/// U+00EB LATIN SMALL LETTER E WITH DIAERESIS
pub const FCITX_KEY_ediaeresis: u32 = 0x00eb;

/// U+00EC LATIN SMALL LETTER I WITH GRAVE
pub const FCITX_KEY_igrave: u32 = 0x00ec;

/// U+00ED LATIN SMALL LETTER I WITH ACUTE
pub const FCITX_KEY_iacute: u32 = 0x00ed;

/// U+00EE LATIN SMALL LETTER I WITH CIRCUMFLEX
pub const FCITX_KEY_icircumflex: u32 = 0x00ee;

/// U+00EF LATIN SMALL LETTER I WITH DIAERESIS
pub const FCITX_KEY_idiaeresis: u32 = 0x00ef;

/// U+00F0 LATIN SMALL LETTER ETH
pub const FCITX_KEY_eth: u32 = 0x00f0;

/// U+00F1 LATIN SMALL LETTER N WITH TILDE
pub const FCITX_KEY_ntilde: u32 = 0x00f1;

/// U+00F2 LATIN SMALL LETTER O WITH GRAVE
pub const FCITX_KEY_ograve: u32 = 0x00f2;

/// U+00F3 LATIN SMALL LETTER O WITH ACUTE
pub const FCITX_KEY_oacute: u32 = 0x00f3;

/// U+00F4 LATIN SMALL LETTER O WITH CIRCUMFLEX
pub const FCITX_KEY_ocircumflex: u32 = 0x00f4;

/// U+00F5 LATIN SMALL LETTER O WITH TILDE
pub const FCITX_KEY_otilde: u32 = 0x00f5;

/// U+00F6 LATIN SMALL LETTER O WITH DIAERESIS
pub const FCITX_KEY_odiaeresis: u32 = 0x00f6;

/// U+00F7 DIVISION SIGN
pub const FCITX_KEY_division: u32 = 0x00f7;

/// U+00F8 LATIN SMALL LETTER O WITH STROKE
pub const FCITX_KEY_oslash: u32 = 0x00f8;

/// deprecated alias for oslash
pub const FCITX_KEY_ooblique: u32 = 0x00f8;

/// U+00F9 LATIN SMALL LETTER U WITH GRAVE
pub const FCITX_KEY_ugrave: u32 = 0x00f9;

/// U+00FA LATIN SMALL LETTER U WITH ACUTE
pub const FCITX_KEY_uacute: u32 = 0x00fa;

/// U+00FB LATIN SMALL LETTER U WITH CIRCUMFLEX
pub const FCITX_KEY_ucircumflex: u32 = 0x00fb;

/// U+00FC LATIN SMALL LETTER U WITH DIAERESIS
pub const FCITX_KEY_udiaeresis: u32 = 0x00fc;

/// U+00FD LATIN SMALL LETTER Y WITH ACUTE
pub const FCITX_KEY_yacute: u32 = 0x00fd;

/// U+00FE LATIN SMALL LETTER THORN
pub const FCITX_KEY_thorn: u32 = 0x00fe;

/// U+00FF LATIN SMALL LETTER Y WITH DIAERESIS
pub const FCITX_KEY_ydiaeresis: u32 = 0x00ff;

/// U+0104 LATIN CAPITAL LETTER A WITH OGONEK
pub const FCITX_KEY_Aogonek: u32 = 0x01a1;

/// U+02D8 BREVE
pub const FCITX_KEY_breve: u32 = 0x01a2;

/// U+0141 LATIN CAPITAL LETTER L WITH STROKE
pub const FCITX_KEY_Lstroke: u32 = 0x01a3;

/// U+013D LATIN CAPITAL LETTER L WITH CARON
pub const FCITX_KEY_Lcaron: u32 = 0x01a5;

/// U+015A LATIN CAPITAL LETTER S WITH ACUTE
pub const FCITX_KEY_Sacute: u32 = 0x01a6;

/// U+0160 LATIN CAPITAL LETTER S WITH CARON
pub const FCITX_KEY_Scaron: u32 = 0x01a9;

/// U+015E LATIN CAPITAL LETTER S WITH CEDILLA
pub const FCITX_KEY_Scedilla: u32 = 0x01aa;

/// U+0164 LATIN CAPITAL LETTER T WITH CARON
pub const FCITX_KEY_Tcaron: u32 = 0x01ab;

/// U+0179 LATIN CAPITAL LETTER Z WITH ACUTE
pub const FCITX_KEY_Zacute: u32 = 0x01ac;

/// U+017D LATIN CAPITAL LETTER Z WITH CARON
pub const FCITX_KEY_Zcaron: u32 = 0x01ae;

/// U+017B LATIN CAPITAL LETTER Z WITH DOT ABOVE
pub const FCITX_KEY_Zabovedot: u32 = 0x01af;

/// U+0105 LATIN SMALL LETTER A WITH OGONEK
pub const FCITX_KEY_aogonek: u32 = 0x01b1;

/// U+02DB OGONEK
pub const FCITX_KEY_ogonek: u32 = 0x01b2;

/// U+0142 LATIN SMALL LETTER L WITH STROKE
pub const FCITX_KEY_lstroke: u32 = 0x01b3;

/// U+013E LATIN SMALL LETTER L WITH CARON
pub const FCITX_KEY_lcaron: u32 = 0x01b5;

/// U+015B LATIN SMALL LETTER S WITH ACUTE
pub const FCITX_KEY_sacute: u32 = 0x01b6;

/// U+02C7 CARON
pub const FCITX_KEY_caron: u32 = 0x01b7;

/// U+0161 LATIN SMALL LETTER S WITH CARON
pub const FCITX_KEY_scaron: u32 = 0x01b9;

/// U+015F LATIN SMALL LETTER S WITH CEDILLA
pub const FCITX_KEY_scedilla: u32 = 0x01ba;

/// U+0165 LATIN SMALL LETTER T WITH CARON
pub const FCITX_KEY_tcaron: u32 = 0x01bb;

/// U+017A LATIN SMALL LETTER Z WITH ACUTE
pub const FCITX_KEY_zacute: u32 = 0x01bc;

/// U+02DD DOUBLE ACUTE ACCENT
pub const FCITX_KEY_doubleacute: u32 = 0x01bd;

/// U+017E LATIN SMALL LETTER Z WITH CARON
pub const FCITX_KEY_zcaron: u32 = 0x01be;

/// U+017C LATIN SMALL LETTER Z WITH DOT ABOVE
pub const FCITX_KEY_zabovedot: u32 = 0x01bf;

/// U+0154 LATIN CAPITAL LETTER R WITH ACUTE
pub const FCITX_KEY_Racute: u32 = 0x01c0;

/// U+0102 LATIN CAPITAL LETTER A WITH BREVE
pub const FCITX_KEY_Abreve: u32 = 0x01c3;

/// U+0139 LATIN CAPITAL LETTER L WITH ACUTE
pub const FCITX_KEY_Lacute: u32 = 0x01c5;

/// U+0106 LATIN CAPITAL LETTER C WITH ACUTE
pub const FCITX_KEY_Cacute: u32 = 0x01c6;

/// U+010C LATIN CAPITAL LETTER C WITH CARON
pub const FCITX_KEY_Ccaron: u32 = 0x01c8;

/// U+0118 LATIN CAPITAL LETTER E WITH OGONEK
pub const FCITX_KEY_Eogonek: u32 = 0x01ca;

/// U+011A LATIN CAPITAL LETTER E WITH CARON
pub const FCITX_KEY_Ecaron: u32 = 0x01cc;

/// U+010E LATIN CAPITAL LETTER D WITH CARON
pub const FCITX_KEY_Dcaron: u32 = 0x01cf;

/// U+0110 LATIN CAPITAL LETTER D WITH STROKE
pub const FCITX_KEY_Dstroke: u32 = 0x01d0;

/// U+0143 LATIN CAPITAL LETTER N WITH ACUTE
pub const FCITX_KEY_Nacute: u32 = 0x01d1;

/// U+0147 LATIN CAPITAL LETTER N WITH CARON
pub const FCITX_KEY_Ncaron: u32 = 0x01d2;

/// U+0150 LATIN CAPITAL LETTER O WITH DOUBLE ACUTE
pub const FCITX_KEY_Odoubleacute: u32 = 0x01d5;

/// U+0158 LATIN CAPITAL LETTER R WITH CARON
pub const FCITX_KEY_Rcaron: u32 = 0x01d8;

/// U+016E LATIN CAPITAL LETTER U WITH RING ABOVE
pub const FCITX_KEY_Uring: u32 = 0x01d9;

/// U+0170 LATIN CAPITAL LETTER U WITH DOUBLE ACUTE
pub const FCITX_KEY_Udoubleacute: u32 = 0x01db;

/// U+0162 LATIN CAPITAL LETTER T WITH CEDILLA
pub const FCITX_KEY_Tcedilla: u32 = 0x01de;

/// U+0155 LATIN SMALL LETTER R WITH ACUTE
pub const FCITX_KEY_racute: u32 = 0x01e0;

/// U+0103 LATIN SMALL LETTER A WITH BREVE
pub const FCITX_KEY_abreve: u32 = 0x01e3;

/// U+013A LATIN SMALL LETTER L WITH ACUTE
pub const FCITX_KEY_lacute: u32 = 0x01e5;

/// U+0107 LATIN SMALL LETTER C WITH ACUTE
pub const FCITX_KEY_cacute: u32 = 0x01e6;

/// U+010D LATIN SMALL LETTER C WITH CARON
pub const FCITX_KEY_ccaron: u32 = 0x01e8;

/// U+0119 LATIN SMALL LETTER E WITH OGONEK
pub const FCITX_KEY_eogonek: u32 = 0x01ea;

/// U+011B LATIN SMALL LETTER E WITH CARON
pub const FCITX_KEY_ecaron: u32 = 0x01ec;

/// U+010F LATIN SMALL LETTER D WITH CARON
pub const FCITX_KEY_dcaron: u32 = 0x01ef;

/// U+0111 LATIN SMALL LETTER D WITH STROKE
pub const FCITX_KEY_dstroke: u32 = 0x01f0;

/// U+0144 LATIN SMALL LETTER N WITH ACUTE
pub const FCITX_KEY_nacute: u32 = 0x01f1;

/// U+0148 LATIN SMALL LETTER N WITH CARON
pub const FCITX_KEY_ncaron: u32 = 0x01f2;

/// U+0151 LATIN SMALL LETTER O WITH DOUBLE ACUTE
pub const FCITX_KEY_odoubleacute: u32 = 0x01f5;

/// U+0159 LATIN SMALL LETTER R WITH CARON
pub const FCITX_KEY_rcaron: u32 = 0x01f8;

/// U+016F LATIN SMALL LETTER U WITH RING ABOVE
pub const FCITX_KEY_uring: u32 = 0x01f9;

/// U+0171 LATIN SMALL LETTER U WITH DOUBLE ACUTE
pub const FCITX_KEY_udoubleacute: u32 = 0x01fb;

/// U+0163 LATIN SMALL LETTER T WITH CEDILLA
pub const FCITX_KEY_tcedilla: u32 = 0x01fe;

/// U+02D9 DOT ABOVE
pub const FCITX_KEY_abovedot: u32 = 0x01ff;

/// U+0126 LATIN CAPITAL LETTER H WITH STROKE
pub const FCITX_KEY_Hstroke: u32 = 0x02a1;

/// U+0124 LATIN CAPITAL LETTER H WITH CIRCUMFLEX
pub const FCITX_KEY_Hcircumflex: u32 = 0x02a6;

/// U+0130 LATIN CAPITAL LETTER I WITH DOT ABOVE
pub const FCITX_KEY_Iabovedot: u32 = 0x02a9;

/// U+011E LATIN CAPITAL LETTER G WITH BREVE
pub const FCITX_KEY_Gbreve: u32 = 0x02ab;

/// U+0134 LATIN CAPITAL LETTER J WITH CIRCUMFLEX
pub const FCITX_KEY_Jcircumflex: u32 = 0x02ac;

/// U+0127 LATIN SMALL LETTER H WITH STROKE
pub const FCITX_KEY_hstroke: u32 = 0x02b1;

/// U+0125 LATIN SMALL LETTER H WITH CIRCUMFLEX
pub const FCITX_KEY_hcircumflex: u32 = 0x02b6;

/// U+0131 LATIN SMALL LETTER DOTLESS I
pub const FCITX_KEY_idotless: u32 = 0x02b9;

/// U+011F LATIN SMALL LETTER G WITH BREVE
pub const FCITX_KEY_gbreve: u32 = 0x02bb;

/// U+0135 LATIN SMALL LETTER J WITH CIRCUMFLEX
pub const FCITX_KEY_jcircumflex: u32 = 0x02bc;

/// U+010A LATIN CAPITAL LETTER C WITH DOT ABOVE
pub const FCITX_KEY_Cabovedot: u32 = 0x02c5;

/// U+0108 LATIN CAPITAL LETTER C WITH CIRCUMFLEX
pub const FCITX_KEY_Ccircumflex: u32 = 0x02c6;

/// U+0120 LATIN CAPITAL LETTER G WITH DOT ABOVE
pub const FCITX_KEY_Gabovedot: u32 = 0x02d5;

/// U+011C LATIN CAPITAL LETTER G WITH CIRCUMFLEX
pub const FCITX_KEY_Gcircumflex: u32 = 0x02d8;

/// U+016C LATIN CAPITAL LETTER U WITH BREVE
pub const FCITX_KEY_Ubreve: u32 = 0x02dd;

/// U+015C LATIN CAPITAL LETTER S WITH CIRCUMFLEX
pub const FCITX_KEY_Scircumflex: u32 = 0x02de;

/// U+010B LATIN SMALL LETTER C WITH DOT ABOVE
pub const FCITX_KEY_cabovedot: u32 = 0x02e5;

/// U+0109 LATIN SMALL LETTER C WITH CIRCUMFLEX
pub const FCITX_KEY_ccircumflex: u32 = 0x02e6;

/// U+0121 LATIN SMALL LETTER G WITH DOT ABOVE
pub const FCITX_KEY_gabovedot: u32 = 0x02f5;

/// U+011D LATIN SMALL LETTER G WITH CIRCUMFLEX
pub const FCITX_KEY_gcircumflex: u32 = 0x02f8;

/// U+016D LATIN SMALL LETTER U WITH BREVE
pub const FCITX_KEY_ubreve: u32 = 0x02fd;

/// U+015D LATIN SMALL LETTER S WITH CIRCUMFLEX
pub const FCITX_KEY_scircumflex: u32 = 0x02fe;

/// U+0138 LATIN SMALL LETTER KRA
pub const FCITX_KEY_kra: u32 = 0x03a2;

/// deprecated
pub const FCITX_KEY_kappa: u32 = 0x03a2;

/// U+0156 LATIN CAPITAL LETTER R WITH CEDILLA
pub const FCITX_KEY_Rcedilla: u32 = 0x03a3;

/// U+0128 LATIN CAPITAL LETTER I WITH TILDE
pub const FCITX_KEY_Itilde: u32 = 0x03a5;

/// U+013B LATIN CAPITAL LETTER L WITH CEDILLA
pub const FCITX_KEY_Lcedilla: u32 = 0x03a6;

/// U+0112 LATIN CAPITAL LETTER E WITH MACRON
pub const FCITX_KEY_Emacron: u32 = 0x03aa;

/// U+0122 LATIN CAPITAL LETTER G WITH CEDILLA
pub const FCITX_KEY_Gcedilla: u32 = 0x03ab;

/// U+0166 LATIN CAPITAL LETTER T WITH STROKE
pub const FCITX_KEY_Tslash: u32 = 0x03ac;

/// U+0157 LATIN SMALL LETTER R WITH CEDILLA
pub const FCITX_KEY_rcedilla: u32 = 0x03b3;

/// U+0129 LATIN SMALL LETTER I WITH TILDE
pub const FCITX_KEY_itilde: u32 = 0x03b5;

/// U+013C LATIN SMALL LETTER L WITH CEDILLA
pub const FCITX_KEY_lcedilla: u32 = 0x03b6;

/// U+0113 LATIN SMALL LETTER E WITH MACRON
pub const FCITX_KEY_emacron: u32 = 0x03ba;

/// U+0123 LATIN SMALL LETTER G WITH CEDILLA
pub const FCITX_KEY_gcedilla: u32 = 0x03bb;

/// U+0167 LATIN SMALL LETTER T WITH STROKE
pub const FCITX_KEY_tslash: u32 = 0x03bc;

/// U+014A LATIN CAPITAL LETTER ENG
pub const FCITX_KEY_ENG: u32 = 0x03bd;

/// U+014B LATIN SMALL LETTER ENG
pub const FCITX_KEY_eng: u32 = 0x03bf;

/// U+0100 LATIN CAPITAL LETTER A WITH MACRON
pub const FCITX_KEY_Amacron: u32 = 0x03c0;

/// U+012E LATIN CAPITAL LETTER I WITH OGONEK
pub const FCITX_KEY_Iogonek: u32 = 0x03c7;

/// U+0116 LATIN CAPITAL LETTER E WITH DOT ABOVE
pub const FCITX_KEY_Eabovedot: u32 = 0x03cc;

/// U+012A LATIN CAPITAL LETTER I WITH MACRON
pub const FCITX_KEY_Imacron: u32 = 0x03cf;

/// U+0145 LATIN CAPITAL LETTER N WITH CEDILLA
pub const FCITX_KEY_Ncedilla: u32 = 0x03d1;

/// U+014C LATIN CAPITAL LETTER O WITH MACRON
pub const FCITX_KEY_Omacron: u32 = 0x03d2;

/// U+0136 LATIN CAPITAL LETTER K WITH CEDILLA
pub const FCITX_KEY_Kcedilla: u32 = 0x03d3;

/// U+0172 LATIN CAPITAL LETTER U WITH OGONEK
pub const FCITX_KEY_Uogonek: u32 = 0x03d9;

/// U+0168 LATIN CAPITAL LETTER U WITH TILDE
pub const FCITX_KEY_Utilde: u32 = 0x03dd;

/// U+016A LATIN CAPITAL LETTER U WITH MACRON
pub const FCITX_KEY_Umacron: u32 = 0x03de;

/// U+0101 LATIN SMALL LETTER A WITH MACRON
pub const FCITX_KEY_amacron: u32 = 0x03e0;

/// U+012F LATIN SMALL LETTER I WITH OGONEK
pub const FCITX_KEY_iogonek: u32 = 0x03e7;

/// U+0117 LATIN SMALL LETTER E WITH DOT ABOVE
pub const FCITX_KEY_eabovedot: u32 = 0x03ec;

/// U+012B LATIN SMALL LETTER I WITH MACRON
pub const FCITX_KEY_imacron: u32 = 0x03ef;

/// U+0146 LATIN SMALL LETTER N WITH CEDILLA
pub const FCITX_KEY_ncedilla: u32 = 0x03f1;

/// U+014D LATIN SMALL LETTER O WITH MACRON
pub const FCITX_KEY_omacron: u32 = 0x03f2;

/// U+0137 LATIN SMALL LETTER K WITH CEDILLA
pub const FCITX_KEY_kcedilla: u32 = 0x03f3;

/// U+0173 LATIN SMALL LETTER U WITH OGONEK
pub const FCITX_KEY_uogonek: u32 = 0x03f9;

/// U+0169 LATIN SMALL LETTER U WITH TILDE
pub const FCITX_KEY_utilde: u32 = 0x03fd;

/// U+016B LATIN SMALL LETTER U WITH MACRON
pub const FCITX_KEY_umacron: u32 = 0x03fe;

/// U+0174 LATIN CAPITAL LETTER W WITH CIRCUMFLEX
pub const FCITX_KEY_Wcircumflex: u32 = 0x1000174;

/// U+0175 LATIN SMALL LETTER W WITH CIRCUMFLEX
pub const FCITX_KEY_wcircumflex: u32 = 0x1000175;

/// U+0176 LATIN CAPITAL LETTER Y WITH CIRCUMFLEX
pub const FCITX_KEY_Ycircumflex: u32 = 0x1000176;

/// U+0177 LATIN SMALL LETTER Y WITH CIRCUMFLEX
pub const FCITX_KEY_ycircumflex: u32 = 0x1000177;

/// U+1E02 LATIN CAPITAL LETTER B WITH DOT ABOVE
pub const FCITX_KEY_Babovedot: u32 = 0x1001e02;

/// U+1E03 LATIN SMALL LETTER B WITH DOT ABOVE
pub const FCITX_KEY_babovedot: u32 = 0x1001e03;

/// U+1E0A LATIN CAPITAL LETTER D WITH DOT ABOVE
pub const FCITX_KEY_Dabovedot: u32 = 0x1001e0a;

/// U+1E0B LATIN SMALL LETTER D WITH DOT ABOVE
pub const FCITX_KEY_dabovedot: u32 = 0x1001e0b;

/// U+1E1E LATIN CAPITAL LETTER F WITH DOT ABOVE
pub const FCITX_KEY_Fabovedot: u32 = 0x1001e1e;

/// U+1E1F LATIN SMALL LETTER F WITH DOT ABOVE
pub const FCITX_KEY_fabovedot: u32 = 0x1001e1f;

/// U+1E40 LATIN CAPITAL LETTER M WITH DOT ABOVE
pub const FCITX_KEY_Mabovedot: u32 = 0x1001e40;

/// U+1E41 LATIN SMALL LETTER M WITH DOT ABOVE
pub const FCITX_KEY_mabovedot: u32 = 0x1001e41;

/// U+1E56 LATIN CAPITAL LETTER P WITH DOT ABOVE
pub const FCITX_KEY_Pabovedot: u32 = 0x1001e56;

/// U+1E57 LATIN SMALL LETTER P WITH DOT ABOVE
pub const FCITX_KEY_pabovedot: u32 = 0x1001e57;

/// U+1E60 LATIN CAPITAL LETTER S WITH DOT ABOVE
pub const FCITX_KEY_Sabovedot: u32 = 0x1001e60;

/// U+1E61 LATIN SMALL LETTER S WITH DOT ABOVE
pub const FCITX_KEY_sabovedot: u32 = 0x1001e61;

/// U+1E6A LATIN CAPITAL LETTER T WITH DOT ABOVE
pub const FCITX_KEY_Tabovedot: u32 = 0x1001e6a;

/// U+1E6B LATIN SMALL LETTER T WITH DOT ABOVE
pub const FCITX_KEY_tabovedot: u32 = 0x1001e6b;

/// U+1E80 LATIN CAPITAL LETTER W WITH GRAVE
pub const FCITX_KEY_Wgrave: u32 = 0x1001e80;

/// U+1E81 LATIN SMALL LETTER W WITH GRAVE
pub const FCITX_KEY_wgrave: u32 = 0x1001e81;

/// U+1E82 LATIN CAPITAL LETTER W WITH ACUTE
pub const FCITX_KEY_Wacute: u32 = 0x1001e82;

/// U+1E83 LATIN SMALL LETTER W WITH ACUTE
pub const FCITX_KEY_wacute: u32 = 0x1001e83;

/// U+1E84 LATIN CAPITAL LETTER W WITH DIAERESIS
pub const FCITX_KEY_Wdiaeresis: u32 = 0x1001e84;

/// U+1E85 LATIN SMALL LETTER W WITH DIAERESIS
pub const FCITX_KEY_wdiaeresis: u32 = 0x1001e85;

/// U+1EF2 LATIN CAPITAL LETTER Y WITH GRAVE
pub const FCITX_KEY_Ygrave: u32 = 0x1001ef2;

/// U+1EF3 LATIN SMALL LETTER Y WITH GRAVE
pub const FCITX_KEY_ygrave: u32 = 0x1001ef3;

/// U+0152 LATIN CAPITAL LIGATURE OE
pub const FCITX_KEY_OE: u32 = 0x13bc;

/// U+0153 LATIN SMALL LIGATURE OE
pub const FCITX_KEY_oe: u32 = 0x13bd;

/// U+0178 LATIN CAPITAL LETTER Y WITH DIAERESIS
pub const FCITX_KEY_Ydiaeresis: u32 = 0x13be;

/// U+203E OVERLINE
pub const FCITX_KEY_overline: u32 = 0x047e;

/// U+3002 IDEOGRAPHIC FULL STOP
pub const FCITX_KEY_kana_fullstop: u32 = 0x04a1;

/// U+300C LEFT CORNER BRACKET
pub const FCITX_KEY_kana_openingbracket: u32 = 0x04a2;

/// U+300D RIGHT CORNER BRACKET
pub const FCITX_KEY_kana_closingbracket: u32 = 0x04a3;

/// U+3001 IDEOGRAPHIC COMMA
pub const FCITX_KEY_kana_comma: u32 = 0x04a4;

/// U+30FB KATAKANA MIDDLE DOT
pub const FCITX_KEY_kana_conjunctive: u32 = 0x04a5;

/// deprecated
pub const FCITX_KEY_kana_middledot: u32 = 0x04a5;

/// U+30F2 KATAKANA LETTER WO
pub const FCITX_KEY_kana_WO: u32 = 0x04a6;

/// U+30A1 KATAKANA LETTER SMALL A
pub const FCITX_KEY_kana_a: u32 = 0x04a7;

/// U+30A3 KATAKANA LETTER SMALL I
pub const FCITX_KEY_kana_i: u32 = 0x04a8;

/// U+30A5 KATAKANA LETTER SMALL U
pub const FCITX_KEY_kana_u: u32 = 0x04a9;

/// U+30A7 KATAKANA LETTER SMALL E
pub const FCITX_KEY_kana_e: u32 = 0x04aa;

/// U+30A9 KATAKANA LETTER SMALL O
pub const FCITX_KEY_kana_o: u32 = 0x04ab;

/// U+30E3 KATAKANA LETTER SMALL YA
pub const FCITX_KEY_kana_ya: u32 = 0x04ac;

/// U+30E5 KATAKANA LETTER SMALL YU
pub const FCITX_KEY_kana_yu: u32 = 0x04ad;

/// U+30E7 KATAKANA LETTER SMALL YO
pub const FCITX_KEY_kana_yo: u32 = 0x04ae;

/// U+30C3 KATAKANA LETTER SMALL TU
pub const FCITX_KEY_kana_tsu: u32 = 0x04af;

/// deprecated
pub const FCITX_KEY_kana_tu: u32 = 0x04af;

/// U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK
pub const FCITX_KEY_prolongedsound: u32 = 0x04b0;

/// U+30A2 KATAKANA LETTER A
pub const FCITX_KEY_kana_A: u32 = 0x04b1;

/// U+30A4 KATAKANA LETTER I
pub const FCITX_KEY_kana_I: u32 = 0x04b2;

/// U+30A6 KATAKANA LETTER U
pub const FCITX_KEY_kana_U: u32 = 0x04b3;

/// U+30A8 KATAKANA LETTER E
pub const FCITX_KEY_kana_E: u32 = 0x04b4;

/// U+30AA KATAKANA LETTER O
pub const FCITX_KEY_kana_O: u32 = 0x04b5;

/// U+30AB KATAKANA LETTER KA
pub const FCITX_KEY_kana_KA: u32 = 0x04b6;

/// U+30AD KATAKANA LETTER KI
pub const FCITX_KEY_kana_KI: u32 = 0x04b7;

/// U+30AF KATAKANA LETTER KU
pub const FCITX_KEY_kana_KU: u32 = 0x04b8;

/// U+30B1 KATAKANA LETTER KE
pub const FCITX_KEY_kana_KE: u32 = 0x04b9;

/// U+30B3 KATAKANA LETTER KO
pub const FCITX_KEY_kana_KO: u32 = 0x04ba;

/// U+30B5 KATAKANA LETTER SA
pub const FCITX_KEY_kana_SA: u32 = 0x04bb;

/// U+30B7 KATAKANA LETTER SI
pub const FCITX_KEY_kana_SHI: u32 = 0x04bc;

/// U+30B9 KATAKANA LETTER SU
pub const FCITX_KEY_kana_SU: u32 = 0x04bd;

/// U+30BB KATAKANA LETTER SE
pub const FCITX_KEY_kana_SE: u32 = 0x04be;

/// U+30BD KATAKANA LETTER SO
pub const FCITX_KEY_kana_SO: u32 = 0x04bf;

/// U+30BF KATAKANA LETTER TA
pub const FCITX_KEY_kana_TA: u32 = 0x04c0;

/// U+30C1 KATAKANA LETTER TI
pub const FCITX_KEY_kana_CHI: u32 = 0x04c1;

/// deprecated
pub const FCITX_KEY_kana_TI: u32 = 0x04c1;

/// U+30C4 KATAKANA LETTER TU
pub const FCITX_KEY_kana_TSU: u32 = 0x04c2;

/// deprecated
pub const FCITX_KEY_kana_TU: u32 = 0x04c2;

/// U+30C6 KATAKANA LETTER TE
pub const FCITX_KEY_kana_TE: u32 = 0x04c3;

/// U+30C8 KATAKANA LETTER TO
pub const FCITX_KEY_kana_TO: u32 = 0x04c4;

/// U+30CA KATAKANA LETTER NA
pub const FCITX_KEY_kana_NA: u32 = 0x04c5;

/// U+30CB KATAKANA LETTER NI
pub const FCITX_KEY_kana_NI: u32 = 0x04c6;

/// U+30CC KATAKANA LETTER NU
pub const FCITX_KEY_kana_NU: u32 = 0x04c7;

/// U+30CD KATAKANA LETTER NE
pub const FCITX_KEY_kana_NE: u32 = 0x04c8;

/// U+30CE KATAKANA LETTER NO
pub const FCITX_KEY_kana_NO: u32 = 0x04c9;

/// U+30CF KATAKANA LETTER HA
pub const FCITX_KEY_kana_HA: u32 = 0x04ca;

/// U+30D2 KATAKANA LETTER HI
pub const FCITX_KEY_kana_HI: u32 = 0x04cb;

/// U+30D5 KATAKANA LETTER HU
pub const FCITX_KEY_kana_FU: u32 = 0x04cc;

/// deprecated
pub const FCITX_KEY_kana_HU: u32 = 0x04cc;

/// U+30D8 KATAKANA LETTER HE
pub const FCITX_KEY_kana_HE: u32 = 0x04cd;

/// U+30DB KATAKANA LETTER HO
pub const FCITX_KEY_kana_HO: u32 = 0x04ce;

/// U+30DE KATAKANA LETTER MA
pub const FCITX_KEY_kana_MA: u32 = 0x04cf;

/// U+30DF KATAKANA LETTER MI
pub const FCITX_KEY_kana_MI: u32 = 0x04d0;

/// U+30E0 KATAKANA LETTER MU
pub const FCITX_KEY_kana_MU: u32 = 0x04d1;

/// U+30E1 KATAKANA LETTER ME
pub const FCITX_KEY_kana_ME: u32 = 0x04d2;

/// U+30E2 KATAKANA LETTER MO
pub const FCITX_KEY_kana_MO: u32 = 0x04d3;

/// U+30E4 KATAKANA LETTER YA
pub const FCITX_KEY_kana_YA: u32 = 0x04d4;

/// U+30E6 KATAKANA LETTER YU
pub const FCITX_KEY_kana_YU: u32 = 0x04d5;

/// U+30E8 KATAKANA LETTER YO
pub const FCITX_KEY_kana_YO: u32 = 0x04d6;

/// U+30E9 KATAKANA LETTER RA
pub const FCITX_KEY_kana_RA: u32 = 0x04d7;

/// U+30EA KATAKANA LETTER RI
pub const FCITX_KEY_kana_RI: u32 = 0x04d8;

/// U+30EB KATAKANA LETTER RU
pub const FCITX_KEY_kana_RU: u32 = 0x04d9;

/// U+30EC KATAKANA LETTER RE
pub const FCITX_KEY_kana_RE: u32 = 0x04da;

/// U+30ED KATAKANA LETTER RO
pub const FCITX_KEY_kana_RO: u32 = 0x04db;

/// U+30EF KATAKANA LETTER WA
pub const FCITX_KEY_kana_WA: u32 = 0x04dc;

/// U+30F3 KATAKANA LETTER N
pub const FCITX_KEY_kana_N: u32 = 0x04dd;

/// U+309B KATAKANA-HIRAGANA VOICED SOUND MARK
pub const FCITX_KEY_voicedsound: u32 = 0x04de;

/// U+309C KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
pub const FCITX_KEY_semivoicedsound: u32 = 0x04df;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_kana_switch: u32 = 0xff7e;

/// U+06F0 EXTENDED ARABIC-INDIC DIGIT ZERO
pub const FCITX_KEY_Farsi_0: u32 = 0x10006f0;

/// U+06F1 EXTENDED ARABIC-INDIC DIGIT ONE
pub const FCITX_KEY_Farsi_1: u32 = 0x10006f1;

/// U+06F2 EXTENDED ARABIC-INDIC DIGIT TWO
pub const FCITX_KEY_Farsi_2: u32 = 0x10006f2;

/// U+06F3 EXTENDED ARABIC-INDIC DIGIT THREE
pub const FCITX_KEY_Farsi_3: u32 = 0x10006f3;

/// U+06F4 EXTENDED ARABIC-INDIC DIGIT FOUR
pub const FCITX_KEY_Farsi_4: u32 = 0x10006f4;

/// U+06F5 EXTENDED ARABIC-INDIC DIGIT FIVE
pub const FCITX_KEY_Farsi_5: u32 = 0x10006f5;

/// U+06F6 EXTENDED ARABIC-INDIC DIGIT SIX
pub const FCITX_KEY_Farsi_6: u32 = 0x10006f6;

/// U+06F7 EXTENDED ARABIC-INDIC DIGIT SEVEN
pub const FCITX_KEY_Farsi_7: u32 = 0x10006f7;

/// U+06F8 EXTENDED ARABIC-INDIC DIGIT EIGHT
pub const FCITX_KEY_Farsi_8: u32 = 0x10006f8;

/// U+06F9 EXTENDED ARABIC-INDIC DIGIT NINE
pub const FCITX_KEY_Farsi_9: u32 = 0x10006f9;

/// U+066A ARABIC PERCENT SIGN
pub const FCITX_KEY_Arabic_percent: u32 = 0x100066a;

/// U+0670 ARABIC LETTER SUPERSCRIPT ALEF
pub const FCITX_KEY_Arabic_superscript_alef: u32 = 0x1000670;

/// U+0679 ARABIC LETTER TTEH
pub const FCITX_KEY_Arabic_tteh: u32 = 0x1000679;

/// U+067E ARABIC LETTER PEH
pub const FCITX_KEY_Arabic_peh: u32 = 0x100067e;

/// U+0686 ARABIC LETTER TCHEH
pub const FCITX_KEY_Arabic_tcheh: u32 = 0x1000686;

/// U+0688 ARABIC LETTER DDAL
pub const FCITX_KEY_Arabic_ddal: u32 = 0x1000688;

/// U+0691 ARABIC LETTER RREH
pub const FCITX_KEY_Arabic_rreh: u32 = 0x1000691;

/// U+060C ARABIC COMMA
pub const FCITX_KEY_Arabic_comma: u32 = 0x05ac;

/// U+06D4 ARABIC FULL STOP
pub const FCITX_KEY_Arabic_fullstop: u32 = 0x10006d4;

/// U+0660 ARABIC-INDIC DIGIT ZERO
pub const FCITX_KEY_Arabic_0: u32 = 0x1000660;

/// U+0661 ARABIC-INDIC DIGIT ONE
pub const FCITX_KEY_Arabic_1: u32 = 0x1000661;

/// U+0662 ARABIC-INDIC DIGIT TWO
pub const FCITX_KEY_Arabic_2: u32 = 0x1000662;

/// U+0663 ARABIC-INDIC DIGIT THREE
pub const FCITX_KEY_Arabic_3: u32 = 0x1000663;

/// U+0664 ARABIC-INDIC DIGIT FOUR
pub const FCITX_KEY_Arabic_4: u32 = 0x1000664;

/// U+0665 ARABIC-INDIC DIGIT FIVE
pub const FCITX_KEY_Arabic_5: u32 = 0x1000665;

/// U+0666 ARABIC-INDIC DIGIT SIX
pub const FCITX_KEY_Arabic_6: u32 = 0x1000666;

/// U+0667 ARABIC-INDIC DIGIT SEVEN
pub const FCITX_KEY_Arabic_7: u32 = 0x1000667;

/// U+0668 ARABIC-INDIC DIGIT EIGHT
pub const FCITX_KEY_Arabic_8: u32 = 0x1000668;

/// U+0669 ARABIC-INDIC DIGIT NINE
pub const FCITX_KEY_Arabic_9: u32 = 0x1000669;

/// U+061B ARABIC SEMICOLON
pub const FCITX_KEY_Arabic_semicolon: u32 = 0x05bb;

/// U+061F ARABIC QUESTION MARK
pub const FCITX_KEY_Arabic_question_mark: u32 = 0x05bf;

/// U+0621 ARABIC LETTER HAMZA
pub const FCITX_KEY_Arabic_hamza: u32 = 0x05c1;

/// U+0622 ARABIC LETTER ALEF WITH MADDA ABOVE
pub const FCITX_KEY_Arabic_maddaonalef: u32 = 0x05c2;

/// U+0623 ARABIC LETTER ALEF WITH HAMZA ABOVE
pub const FCITX_KEY_Arabic_hamzaonalef: u32 = 0x05c3;

/// U+0624 ARABIC LETTER WAW WITH HAMZA ABOVE
pub const FCITX_KEY_Arabic_hamzaonwaw: u32 = 0x05c4;

/// U+0625 ARABIC LETTER ALEF WITH HAMZA BELOW
pub const FCITX_KEY_Arabic_hamzaunderalef: u32 = 0x05c5;

/// U+0626 ARABIC LETTER YEH WITH HAMZA ABOVE
pub const FCITX_KEY_Arabic_hamzaonyeh: u32 = 0x05c6;

/// U+0627 ARABIC LETTER ALEF
pub const FCITX_KEY_Arabic_alef: u32 = 0x05c7;

/// U+0628 ARABIC LETTER BEH
pub const FCITX_KEY_Arabic_beh: u32 = 0x05c8;

/// U+0629 ARABIC LETTER TEH MARBUTA
pub const FCITX_KEY_Arabic_tehmarbuta: u32 = 0x05c9;

/// U+062A ARABIC LETTER TEH
pub const FCITX_KEY_Arabic_teh: u32 = 0x05ca;

/// U+062B ARABIC LETTER THEH
pub const FCITX_KEY_Arabic_theh: u32 = 0x05cb;

/// U+062C ARABIC LETTER JEEM
pub const FCITX_KEY_Arabic_jeem: u32 = 0x05cc;

/// U+062D ARABIC LETTER HAH
pub const FCITX_KEY_Arabic_hah: u32 = 0x05cd;

/// U+062E ARABIC LETTER KHAH
pub const FCITX_KEY_Arabic_khah: u32 = 0x05ce;

/// U+062F ARABIC LETTER DAL
pub const FCITX_KEY_Arabic_dal: u32 = 0x05cf;

/// U+0630 ARABIC LETTER THAL
pub const FCITX_KEY_Arabic_thal: u32 = 0x05d0;

/// U+0631 ARABIC LETTER REH
pub const FCITX_KEY_Arabic_ra: u32 = 0x05d1;

/// U+0632 ARABIC LETTER ZAIN
pub const FCITX_KEY_Arabic_zain: u32 = 0x05d2;

/// U+0633 ARABIC LETTER SEEN
pub const FCITX_KEY_Arabic_seen: u32 = 0x05d3;

/// U+0634 ARABIC LETTER SHEEN
pub const FCITX_KEY_Arabic_sheen: u32 = 0x05d4;

/// U+0635 ARABIC LETTER SAD
pub const FCITX_KEY_Arabic_sad: u32 = 0x05d5;

/// U+0636 ARABIC LETTER DAD
pub const FCITX_KEY_Arabic_dad: u32 = 0x05d6;

/// U+0637 ARABIC LETTER TAH
pub const FCITX_KEY_Arabic_tah: u32 = 0x05d7;

/// U+0638 ARABIC LETTER ZAH
pub const FCITX_KEY_Arabic_zah: u32 = 0x05d8;

/// U+0639 ARABIC LETTER AIN
pub const FCITX_KEY_Arabic_ain: u32 = 0x05d9;

/// U+063A ARABIC LETTER GHAIN
pub const FCITX_KEY_Arabic_ghain: u32 = 0x05da;

/// U+0640 ARABIC TATWEEL
pub const FCITX_KEY_Arabic_tatweel: u32 = 0x05e0;

/// U+0641 ARABIC LETTER FEH
pub const FCITX_KEY_Arabic_feh: u32 = 0x05e1;

/// U+0642 ARABIC LETTER QAF
pub const FCITX_KEY_Arabic_qaf: u32 = 0x05e2;

/// U+0643 ARABIC LETTER KAF
pub const FCITX_KEY_Arabic_kaf: u32 = 0x05e3;

/// U+0644 ARABIC LETTER LAM
pub const FCITX_KEY_Arabic_lam: u32 = 0x05e4;

/// U+0645 ARABIC LETTER MEEM
pub const FCITX_KEY_Arabic_meem: u32 = 0x05e5;

/// U+0646 ARABIC LETTER NOON
pub const FCITX_KEY_Arabic_noon: u32 = 0x05e6;

/// U+0647 ARABIC LETTER HEH
pub const FCITX_KEY_Arabic_ha: u32 = 0x05e7;

/// deprecated
pub const FCITX_KEY_Arabic_heh: u32 = 0x05e7;

/// U+0648 ARABIC LETTER WAW
pub const FCITX_KEY_Arabic_waw: u32 = 0x05e8;

/// U+0649 ARABIC LETTER ALEF MAKSURA
pub const FCITX_KEY_Arabic_alefmaksura: u32 = 0x05e9;

/// U+064A ARABIC LETTER YEH
pub const FCITX_KEY_Arabic_yeh: u32 = 0x05ea;

/// U+064B ARABIC FATHATAN
pub const FCITX_KEY_Arabic_fathatan: u32 = 0x05eb;

/// U+064C ARABIC DAMMATAN
pub const FCITX_KEY_Arabic_dammatan: u32 = 0x05ec;

/// U+064D ARABIC KASRATAN
pub const FCITX_KEY_Arabic_kasratan: u32 = 0x05ed;

/// U+064E ARABIC FATHA
pub const FCITX_KEY_Arabic_fatha: u32 = 0x05ee;

/// U+064F ARABIC DAMMA
pub const FCITX_KEY_Arabic_damma: u32 = 0x05ef;

/// U+0650 ARABIC KASRA
pub const FCITX_KEY_Arabic_kasra: u32 = 0x05f0;

/// U+0651 ARABIC SHADDA
pub const FCITX_KEY_Arabic_shadda: u32 = 0x05f1;

/// U+0652 ARABIC SUKUN
pub const FCITX_KEY_Arabic_sukun: u32 = 0x05f2;

/// U+0653 ARABIC MADDAH ABOVE
pub const FCITX_KEY_Arabic_madda_above: u32 = 0x1000653;

/// U+0654 ARABIC HAMZA ABOVE
pub const FCITX_KEY_Arabic_hamza_above: u32 = 0x1000654;

/// U+0655 ARABIC HAMZA BELOW
pub const FCITX_KEY_Arabic_hamza_below: u32 = 0x1000655;

/// U+0698 ARABIC LETTER JEH
pub const FCITX_KEY_Arabic_jeh: u32 = 0x1000698;

/// U+06A4 ARABIC LETTER VEH
pub const FCITX_KEY_Arabic_veh: u32 = 0x10006a4;

/// U+06A9 ARABIC LETTER KEHEH
pub const FCITX_KEY_Arabic_keheh: u32 = 0x10006a9;

/// U+06AF ARABIC LETTER GAF
pub const FCITX_KEY_Arabic_gaf: u32 = 0x10006af;

/// U+06BA ARABIC LETTER NOON GHUNNA
pub const FCITX_KEY_Arabic_noon_ghunna: u32 = 0x10006ba;

/// U+06BE ARABIC LETTER HEH DOACHASHMEE
pub const FCITX_KEY_Arabic_heh_doachashmee: u32 = 0x10006be;

/// U+06CC ARABIC LETTER FARSI YEH
pub const FCITX_KEY_Farsi_yeh: u32 = 0x10006cc;

/// deprecated alias for Farsi_yeh
pub const FCITX_KEY_Arabic_farsi_yeh: u32 = 0x10006cc;

/// U+06D2 ARABIC LETTER YEH BARREE
pub const FCITX_KEY_Arabic_yeh_baree: u32 = 0x10006d2;

/// U+06C1 ARABIC LETTER HEH GOAL
pub const FCITX_KEY_Arabic_heh_goal: u32 = 0x10006c1;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_Arabic_switch: u32 = 0xff7e;

/// U+0492 CYRILLIC CAPITAL LETTER GHE WITH STROKE
pub const FCITX_KEY_Cyrillic_GHE_bar: u32 = 0x1000492;

/// U+0493 CYRILLIC SMALL LETTER GHE WITH STROKE
pub const FCITX_KEY_Cyrillic_ghe_bar: u32 = 0x1000493;

/// U+0496 CYRILLIC CAPITAL LETTER ZHE WITH DESCENDER
pub const FCITX_KEY_Cyrillic_ZHE_descender: u32 = 0x1000496;

/// U+0497 CYRILLIC SMALL LETTER ZHE WITH DESCENDER
pub const FCITX_KEY_Cyrillic_zhe_descender: u32 = 0x1000497;

/// U+049A CYRILLIC CAPITAL LETTER KA WITH DESCENDER
pub const FCITX_KEY_Cyrillic_KA_descender: u32 = 0x100049a;

/// U+049B CYRILLIC SMALL LETTER KA WITH DESCENDER
pub const FCITX_KEY_Cyrillic_ka_descender: u32 = 0x100049b;

/// U+049C CYRILLIC CAPITAL LETTER KA WITH VERTICAL STROKE
pub const FCITX_KEY_Cyrillic_KA_vertstroke: u32 = 0x100049c;

/// U+049D CYRILLIC SMALL LETTER KA WITH VERTICAL STROKE
pub const FCITX_KEY_Cyrillic_ka_vertstroke: u32 = 0x100049d;

/// U+04A2 CYRILLIC CAPITAL LETTER EN WITH DESCENDER
pub const FCITX_KEY_Cyrillic_EN_descender: u32 = 0x10004a2;

/// U+04A3 CYRILLIC SMALL LETTER EN WITH DESCENDER
pub const FCITX_KEY_Cyrillic_en_descender: u32 = 0x10004a3;

/// U+04AE CYRILLIC CAPITAL LETTER STRAIGHT U
pub const FCITX_KEY_Cyrillic_U_straight: u32 = 0x10004ae;

/// U+04AF CYRILLIC SMALL LETTER STRAIGHT U
pub const FCITX_KEY_Cyrillic_u_straight: u32 = 0x10004af;

/// U+04B0 CYRILLIC CAPITAL LETTER STRAIGHT U WITH STROKE
pub const FCITX_KEY_Cyrillic_U_straight_bar: u32 = 0x10004b0;

/// U+04B1 CYRILLIC SMALL LETTER STRAIGHT U WITH STROKE
pub const FCITX_KEY_Cyrillic_u_straight_bar: u32 = 0x10004b1;

/// U+04B2 CYRILLIC CAPITAL LETTER HA WITH DESCENDER
pub const FCITX_KEY_Cyrillic_HA_descender: u32 = 0x10004b2;

/// U+04B3 CYRILLIC SMALL LETTER HA WITH DESCENDER
pub const FCITX_KEY_Cyrillic_ha_descender: u32 = 0x10004b3;

/// U+04B6 CYRILLIC CAPITAL LETTER CHE WITH DESCENDER
pub const FCITX_KEY_Cyrillic_CHE_descender: u32 = 0x10004b6;

/// U+04B7 CYRILLIC SMALL LETTER CHE WITH DESCENDER
pub const FCITX_KEY_Cyrillic_che_descender: u32 = 0x10004b7;

/// U+04B8 CYRILLIC CAPITAL LETTER CHE WITH VERTICAL STROKE
pub const FCITX_KEY_Cyrillic_CHE_vertstroke: u32 = 0x10004b8;

/// U+04B9 CYRILLIC SMALL LETTER CHE WITH VERTICAL STROKE
pub const FCITX_KEY_Cyrillic_che_vertstroke: u32 = 0x10004b9;

/// U+04BA CYRILLIC CAPITAL LETTER SHHA
pub const FCITX_KEY_Cyrillic_SHHA: u32 = 0x10004ba;

/// U+04BB CYRILLIC SMALL LETTER SHHA
pub const FCITX_KEY_Cyrillic_shha: u32 = 0x10004bb;

/// U+04D8 CYRILLIC CAPITAL LETTER SCHWA
pub const FCITX_KEY_Cyrillic_SCHWA: u32 = 0x10004d8;

/// U+04D9 CYRILLIC SMALL LETTER SCHWA
pub const FCITX_KEY_Cyrillic_schwa: u32 = 0x10004d9;

/// U+04E2 CYRILLIC CAPITAL LETTER I WITH MACRON
pub const FCITX_KEY_Cyrillic_I_macron: u32 = 0x10004e2;

/// U+04E3 CYRILLIC SMALL LETTER I WITH MACRON
pub const FCITX_KEY_Cyrillic_i_macron: u32 = 0x10004e3;

/// U+04E8 CYRILLIC CAPITAL LETTER BARRED O
pub const FCITX_KEY_Cyrillic_O_bar: u32 = 0x10004e8;

/// U+04E9 CYRILLIC SMALL LETTER BARRED O
pub const FCITX_KEY_Cyrillic_o_bar: u32 = 0x10004e9;

/// U+04EE CYRILLIC CAPITAL LETTER U WITH MACRON
pub const FCITX_KEY_Cyrillic_U_macron: u32 = 0x10004ee;

/// U+04EF CYRILLIC SMALL LETTER U WITH MACRON
pub const FCITX_KEY_Cyrillic_u_macron: u32 = 0x10004ef;

/// U+0452 CYRILLIC SMALL LETTER DJE
pub const FCITX_KEY_Serbian_dje: u32 = 0x06a1;

/// U+0453 CYRILLIC SMALL LETTER GJE
pub const FCITX_KEY_Macedonia_gje: u32 = 0x06a2;

/// U+0451 CYRILLIC SMALL LETTER IO
pub const FCITX_KEY_Cyrillic_io: u32 = 0x06a3;

/// U+0454 CYRILLIC SMALL LETTER UKRAINIAN IE
pub const FCITX_KEY_Ukrainian_ie: u32 = 0x06a4;

/// deprecated
pub const FCITX_KEY_Ukranian_je: u32 = 0x06a4;

/// U+0455 CYRILLIC SMALL LETTER DZE
pub const FCITX_KEY_Macedonia_dse: u32 = 0x06a5;

/// U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
pub const FCITX_KEY_Ukrainian_i: u32 = 0x06a6;

/// deprecated
pub const FCITX_KEY_Ukranian_i: u32 = 0x06a6;

/// U+0457 CYRILLIC SMALL LETTER YI
pub const FCITX_KEY_Ukrainian_yi: u32 = 0x06a7;

/// deprecated
pub const FCITX_KEY_Ukranian_yi: u32 = 0x06a7;

/// U+0458 CYRILLIC SMALL LETTER JE
pub const FCITX_KEY_Cyrillic_je: u32 = 0x06a8;

/// deprecated
pub const FCITX_KEY_Serbian_je: u32 = 0x06a8;

/// U+0459 CYRILLIC SMALL LETTER LJE
pub const FCITX_KEY_Cyrillic_lje: u32 = 0x06a9;

/// deprecated
pub const FCITX_KEY_Serbian_lje: u32 = 0x06a9;

/// U+045A CYRILLIC SMALL LETTER NJE
pub const FCITX_KEY_Cyrillic_nje: u32 = 0x06aa;

/// deprecated
pub const FCITX_KEY_Serbian_nje: u32 = 0x06aa;

/// U+045B CYRILLIC SMALL LETTER TSHE
pub const FCITX_KEY_Serbian_tshe: u32 = 0x06ab;

/// U+045C CYRILLIC SMALL LETTER KJE
pub const FCITX_KEY_Macedonia_kje: u32 = 0x06ac;

/// U+0491 CYRILLIC SMALL LETTER GHE WITH UPTURN
pub const FCITX_KEY_Ukrainian_ghe_with_upturn: u32 = 0x06ad;

/// U+045E CYRILLIC SMALL LETTER SHORT U
pub const FCITX_KEY_Byelorussian_shortu: u32 = 0x06ae;

/// U+045F CYRILLIC SMALL LETTER DZHE
pub const FCITX_KEY_Cyrillic_dzhe: u32 = 0x06af;

/// deprecated
pub const FCITX_KEY_Serbian_dze: u32 = 0x06af;

/// U+2116 NUMERO SIGN
pub const FCITX_KEY_numerosign: u32 = 0x06b0;

/// U+0402 CYRILLIC CAPITAL LETTER DJE
pub const FCITX_KEY_Serbian_DJE: u32 = 0x06b1;

/// U+0403 CYRILLIC CAPITAL LETTER GJE
pub const FCITX_KEY_Macedonia_GJE: u32 = 0x06b2;

/// U+0401 CYRILLIC CAPITAL LETTER IO
pub const FCITX_KEY_Cyrillic_IO: u32 = 0x06b3;

/// U+0404 CYRILLIC CAPITAL LETTER UKRAINIAN IE
pub const FCITX_KEY_Ukrainian_IE: u32 = 0x06b4;

/// deprecated
pub const FCITX_KEY_Ukranian_JE: u32 = 0x06b4;

/// U+0405 CYRILLIC CAPITAL LETTER DZE
pub const FCITX_KEY_Macedonia_DSE: u32 = 0x06b5;

/// U+0406 CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I
pub const FCITX_KEY_Ukrainian_I: u32 = 0x06b6;

/// deprecated
pub const FCITX_KEY_Ukranian_I: u32 = 0x06b6;

/// U+0407 CYRILLIC CAPITAL LETTER YI
pub const FCITX_KEY_Ukrainian_YI: u32 = 0x06b7;

/// deprecated
pub const FCITX_KEY_Ukranian_YI: u32 = 0x06b7;

/// U+0408 CYRILLIC CAPITAL LETTER JE
pub const FCITX_KEY_Cyrillic_JE: u32 = 0x06b8;

/// deprecated
pub const FCITX_KEY_Serbian_JE: u32 = 0x06b8;

/// U+0409 CYRILLIC CAPITAL LETTER LJE
pub const FCITX_KEY_Cyrillic_LJE: u32 = 0x06b9;

/// deprecated
pub const FCITX_KEY_Serbian_LJE: u32 = 0x06b9;

/// U+040A CYRILLIC CAPITAL LETTER NJE
pub const FCITX_KEY_Cyrillic_NJE: u32 = 0x06ba;

/// deprecated
pub const FCITX_KEY_Serbian_NJE: u32 = 0x06ba;

/// U+040B CYRILLIC CAPITAL LETTER TSHE
pub const FCITX_KEY_Serbian_TSHE: u32 = 0x06bb;

/// U+040C CYRILLIC CAPITAL LETTER KJE
pub const FCITX_KEY_Macedonia_KJE: u32 = 0x06bc;

/// U+0490 CYRILLIC CAPITAL LETTER GHE WITH UPTURN
pub const FCITX_KEY_Ukrainian_GHE_WITH_UPTURN: u32 = 0x06bd;

/// U+040E CYRILLIC CAPITAL LETTER SHORT U
pub const FCITX_KEY_Byelorussian_SHORTU: u32 = 0x06be;

/// U+040F CYRILLIC CAPITAL LETTER DZHE
pub const FCITX_KEY_Cyrillic_DZHE: u32 = 0x06bf;

/// deprecated
pub const FCITX_KEY_Serbian_DZE: u32 = 0x06bf;

/// U+044E CYRILLIC SMALL LETTER YU
pub const FCITX_KEY_Cyrillic_yu: u32 = 0x06c0;

/// U+0430 CYRILLIC SMALL LETTER A
pub const FCITX_KEY_Cyrillic_a: u32 = 0x06c1;

/// U+0431 CYRILLIC SMALL LETTER BE
pub const FCITX_KEY_Cyrillic_be: u32 = 0x06c2;

/// U+0446 CYRILLIC SMALL LETTER TSE
pub const FCITX_KEY_Cyrillic_tse: u32 = 0x06c3;

/// U+0434 CYRILLIC SMALL LETTER DE
pub const FCITX_KEY_Cyrillic_de: u32 = 0x06c4;

/// U+0435 CYRILLIC SMALL LETTER IE
pub const FCITX_KEY_Cyrillic_ie: u32 = 0x06c5;

/// U+0444 CYRILLIC SMALL LETTER EF
pub const FCITX_KEY_Cyrillic_ef: u32 = 0x06c6;

/// U+0433 CYRILLIC SMALL LETTER GHE
pub const FCITX_KEY_Cyrillic_ghe: u32 = 0x06c7;

/// U+0445 CYRILLIC SMALL LETTER HA
pub const FCITX_KEY_Cyrillic_ha: u32 = 0x06c8;

/// U+0438 CYRILLIC SMALL LETTER I
pub const FCITX_KEY_Cyrillic_i: u32 = 0x06c9;

/// U+0439 CYRILLIC SMALL LETTER SHORT I
pub const FCITX_KEY_Cyrillic_shorti: u32 = 0x06ca;

/// U+043A CYRILLIC SMALL LETTER KA
pub const FCITX_KEY_Cyrillic_ka: u32 = 0x06cb;

/// U+043B CYRILLIC SMALL LETTER EL
pub const FCITX_KEY_Cyrillic_el: u32 = 0x06cc;

/// U+043C CYRILLIC SMALL LETTER EM
pub const FCITX_KEY_Cyrillic_em: u32 = 0x06cd;

/// U+043D CYRILLIC SMALL LETTER EN
pub const FCITX_KEY_Cyrillic_en: u32 = 0x06ce;

/// U+043E CYRILLIC SMALL LETTER O
pub const FCITX_KEY_Cyrillic_o: u32 = 0x06cf;

/// U+043F CYRILLIC SMALL LETTER PE
pub const FCITX_KEY_Cyrillic_pe: u32 = 0x06d0;

/// U+044F CYRILLIC SMALL LETTER YA
pub const FCITX_KEY_Cyrillic_ya: u32 = 0x06d1;

/// U+0440 CYRILLIC SMALL LETTER ER
pub const FCITX_KEY_Cyrillic_er: u32 = 0x06d2;

/// U+0441 CYRILLIC SMALL LETTER ES
pub const FCITX_KEY_Cyrillic_es: u32 = 0x06d3;

/// U+0442 CYRILLIC SMALL LETTER TE
pub const FCITX_KEY_Cyrillic_te: u32 = 0x06d4;

/// U+0443 CYRILLIC SMALL LETTER U
pub const FCITX_KEY_Cyrillic_u: u32 = 0x06d5;

/// U+0436 CYRILLIC SMALL LETTER ZHE
pub const FCITX_KEY_Cyrillic_zhe: u32 = 0x06d6;

/// U+0432 CYRILLIC SMALL LETTER VE
pub const FCITX_KEY_Cyrillic_ve: u32 = 0x06d7;

/// U+044C CYRILLIC SMALL LETTER SOFT SIGN
pub const FCITX_KEY_Cyrillic_softsign: u32 = 0x06d8;

/// U+044B CYRILLIC SMALL LETTER YERU
pub const FCITX_KEY_Cyrillic_yeru: u32 = 0x06d9;

/// U+0437 CYRILLIC SMALL LETTER ZE
pub const FCITX_KEY_Cyrillic_ze: u32 = 0x06da;

/// U+0448 CYRILLIC SMALL LETTER SHA
pub const FCITX_KEY_Cyrillic_sha: u32 = 0x06db;

/// U+044D CYRILLIC SMALL LETTER E
pub const FCITX_KEY_Cyrillic_e: u32 = 0x06dc;

/// U+0449 CYRILLIC SMALL LETTER SHCHA
pub const FCITX_KEY_Cyrillic_shcha: u32 = 0x06dd;

/// U+0447 CYRILLIC SMALL LETTER CHE
pub const FCITX_KEY_Cyrillic_che: u32 = 0x06de;

/// U+044A CYRILLIC SMALL LETTER HARD SIGN
pub const FCITX_KEY_Cyrillic_hardsign: u32 = 0x06df;

/// U+042E CYRILLIC CAPITAL LETTER YU
pub const FCITX_KEY_Cyrillic_YU: u32 = 0x06e0;

/// U+0410 CYRILLIC CAPITAL LETTER A
pub const FCITX_KEY_Cyrillic_A: u32 = 0x06e1;

/// U+0411 CYRILLIC CAPITAL LETTER BE
pub const FCITX_KEY_Cyrillic_BE: u32 = 0x06e2;

/// U+0426 CYRILLIC CAPITAL LETTER TSE
pub const FCITX_KEY_Cyrillic_TSE: u32 = 0x06e3;

/// U+0414 CYRILLIC CAPITAL LETTER DE
pub const FCITX_KEY_Cyrillic_DE: u32 = 0x06e4;

/// U+0415 CYRILLIC CAPITAL LETTER IE
pub const FCITX_KEY_Cyrillic_IE: u32 = 0x06e5;

/// U+0424 CYRILLIC CAPITAL LETTER EF
pub const FCITX_KEY_Cyrillic_EF: u32 = 0x06e6;

/// U+0413 CYRILLIC CAPITAL LETTER GHE
pub const FCITX_KEY_Cyrillic_GHE: u32 = 0x06e7;

/// U+0425 CYRILLIC CAPITAL LETTER HA
pub const FCITX_KEY_Cyrillic_HA: u32 = 0x06e8;

/// U+0418 CYRILLIC CAPITAL LETTER I
pub const FCITX_KEY_Cyrillic_I: u32 = 0x06e9;

/// U+0419 CYRILLIC CAPITAL LETTER SHORT I
pub const FCITX_KEY_Cyrillic_SHORTI: u32 = 0x06ea;

/// U+041A CYRILLIC CAPITAL LETTER KA
pub const FCITX_KEY_Cyrillic_KA: u32 = 0x06eb;

/// U+041B CYRILLIC CAPITAL LETTER EL
pub const FCITX_KEY_Cyrillic_EL: u32 = 0x06ec;

/// U+041C CYRILLIC CAPITAL LETTER EM
pub const FCITX_KEY_Cyrillic_EM: u32 = 0x06ed;

/// U+041D CYRILLIC CAPITAL LETTER EN
pub const FCITX_KEY_Cyrillic_EN: u32 = 0x06ee;

/// U+041E CYRILLIC CAPITAL LETTER O
pub const FCITX_KEY_Cyrillic_O: u32 = 0x06ef;

/// U+041F CYRILLIC CAPITAL LETTER PE
pub const FCITX_KEY_Cyrillic_PE: u32 = 0x06f0;

/// U+042F CYRILLIC CAPITAL LETTER YA
pub const FCITX_KEY_Cyrillic_YA: u32 = 0x06f1;

/// U+0420 CYRILLIC CAPITAL LETTER ER
pub const FCITX_KEY_Cyrillic_ER: u32 = 0x06f2;

/// U+0421 CYRILLIC CAPITAL LETTER ES
pub const FCITX_KEY_Cyrillic_ES: u32 = 0x06f3;

/// U+0422 CYRILLIC CAPITAL LETTER TE
pub const FCITX_KEY_Cyrillic_TE: u32 = 0x06f4;

/// U+0423 CYRILLIC CAPITAL LETTER U
pub const FCITX_KEY_Cyrillic_U: u32 = 0x06f5;

/// U+0416 CYRILLIC CAPITAL LETTER ZHE
pub const FCITX_KEY_Cyrillic_ZHE: u32 = 0x06f6;

/// U+0412 CYRILLIC CAPITAL LETTER VE
pub const FCITX_KEY_Cyrillic_VE: u32 = 0x06f7;

/// U+042C CYRILLIC CAPITAL LETTER SOFT SIGN
pub const FCITX_KEY_Cyrillic_SOFTSIGN: u32 = 0x06f8;

/// U+042B CYRILLIC CAPITAL LETTER YERU
pub const FCITX_KEY_Cyrillic_YERU: u32 = 0x06f9;

/// U+0417 CYRILLIC CAPITAL LETTER ZE
pub const FCITX_KEY_Cyrillic_ZE: u32 = 0x06fa;

/// U+0428 CYRILLIC CAPITAL LETTER SHA
pub const FCITX_KEY_Cyrillic_SHA: u32 = 0x06fb;

/// U+042D CYRILLIC CAPITAL LETTER E
pub const FCITX_KEY_Cyrillic_E: u32 = 0x06fc;

/// U+0429 CYRILLIC CAPITAL LETTER SHCHA
pub const FCITX_KEY_Cyrillic_SHCHA: u32 = 0x06fd;

/// U+0427 CYRILLIC CAPITAL LETTER CHE
pub const FCITX_KEY_Cyrillic_CHE: u32 = 0x06fe;

/// U+042A CYRILLIC CAPITAL LETTER HARD SIGN
pub const FCITX_KEY_Cyrillic_HARDSIGN: u32 = 0x06ff;

/// U+0386 GREEK CAPITAL LETTER ALPHA WITH TONOS
pub const FCITX_KEY_Greek_ALPHAaccent: u32 = 0x07a1;

/// U+0388 GREEK CAPITAL LETTER EPSILON WITH TONOS
pub const FCITX_KEY_Greek_EPSILONaccent: u32 = 0x07a2;

/// U+0389 GREEK CAPITAL LETTER ETA WITH TONOS
pub const FCITX_KEY_Greek_ETAaccent: u32 = 0x07a3;

/// U+038A GREEK CAPITAL LETTER IOTA WITH TONOS
pub const FCITX_KEY_Greek_IOTAaccent: u32 = 0x07a4;

/// U+03AA GREEK CAPITAL LETTER IOTA WITH DIALYTIKA
pub const FCITX_KEY_Greek_IOTAdieresis: u32 = 0x07a5;

/// deprecated (old typo)
pub const FCITX_KEY_Greek_IOTAdiaeresis: u32 = 0x07a5;

/// U+038C GREEK CAPITAL LETTER OMICRON WITH TONOS
pub const FCITX_KEY_Greek_OMICRONaccent: u32 = 0x07a7;

/// U+038E GREEK CAPITAL LETTER UPSILON WITH TONOS
pub const FCITX_KEY_Greek_UPSILONaccent: u32 = 0x07a8;

/// U+03AB GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA
pub const FCITX_KEY_Greek_UPSILONdieresis: u32 = 0x07a9;

/// U+038F GREEK CAPITAL LETTER OMEGA WITH TONOS
pub const FCITX_KEY_Greek_OMEGAaccent: u32 = 0x07ab;

/// U+0385 GREEK DIALYTIKA TONOS
pub const FCITX_KEY_Greek_accentdieresis: u32 = 0x07ae;

/// U+2015 HORIZONTAL BAR
pub const FCITX_KEY_Greek_horizbar: u32 = 0x07af;

/// U+03AC GREEK SMALL LETTER ALPHA WITH TONOS
pub const FCITX_KEY_Greek_alphaaccent: u32 = 0x07b1;

/// U+03AD GREEK SMALL LETTER EPSILON WITH TONOS
pub const FCITX_KEY_Greek_epsilonaccent: u32 = 0x07b2;

/// U+03AE GREEK SMALL LETTER ETA WITH TONOS
pub const FCITX_KEY_Greek_etaaccent: u32 = 0x07b3;

/// U+03AF GREEK SMALL LETTER IOTA WITH TONOS
pub const FCITX_KEY_Greek_iotaaccent: u32 = 0x07b4;

/// U+03CA GREEK SMALL LETTER IOTA WITH DIALYTIKA
pub const FCITX_KEY_Greek_iotadieresis: u32 = 0x07b5;

/// U+0390 GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS
pub const FCITX_KEY_Greek_iotaaccentdieresis: u32 = 0x07b6;

/// U+03CC GREEK SMALL LETTER OMICRON WITH TONOS
pub const FCITX_KEY_Greek_omicronaccent: u32 = 0x07b7;

/// U+03CD GREEK SMALL LETTER UPSILON WITH TONOS
pub const FCITX_KEY_Greek_upsilonaccent: u32 = 0x07b8;

/// U+03CB GREEK SMALL LETTER UPSILON WITH DIALYTIKA
pub const FCITX_KEY_Greek_upsilondieresis: u32 = 0x07b9;

/// U+03B0 GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS
pub const FCITX_KEY_Greek_upsilonaccentdieresis: u32 = 0x07ba;

/// U+03CE GREEK SMALL LETTER OMEGA WITH TONOS
pub const FCITX_KEY_Greek_omegaaccent: u32 = 0x07bb;

/// U+0391 GREEK CAPITAL LETTER ALPHA
pub const FCITX_KEY_Greek_ALPHA: u32 = 0x07c1;

/// U+0392 GREEK CAPITAL LETTER BETA
pub const FCITX_KEY_Greek_BETA: u32 = 0x07c2;

/// U+0393 GREEK CAPITAL LETTER GAMMA
pub const FCITX_KEY_Greek_GAMMA: u32 = 0x07c3;

/// U+0394 GREEK CAPITAL LETTER DELTA
pub const FCITX_KEY_Greek_DELTA: u32 = 0x07c4;

/// U+0395 GREEK CAPITAL LETTER EPSILON
pub const FCITX_KEY_Greek_EPSILON: u32 = 0x07c5;

/// U+0396 GREEK CAPITAL LETTER ZETA
pub const FCITX_KEY_Greek_ZETA: u32 = 0x07c6;

/// U+0397 GREEK CAPITAL LETTER ETA
pub const FCITX_KEY_Greek_ETA: u32 = 0x07c7;

/// U+0398 GREEK CAPITAL LETTER THETA
pub const FCITX_KEY_Greek_THETA: u32 = 0x07c8;

/// U+0399 GREEK CAPITAL LETTER IOTA
pub const FCITX_KEY_Greek_IOTA: u32 = 0x07c9;

/// U+039A GREEK CAPITAL LETTER KAPPA
pub const FCITX_KEY_Greek_KAPPA: u32 = 0x07ca;

/// U+039B GREEK CAPITAL LETTER LAMDA
pub const FCITX_KEY_Greek_LAMDA: u32 = 0x07cb;

/// non-deprecated alias for Greek_LAMDA
pub const FCITX_KEY_Greek_LAMBDA: u32 = 0x07cb;

/// U+039C GREEK CAPITAL LETTER MU
pub const FCITX_KEY_Greek_MU: u32 = 0x07cc;

/// U+039D GREEK CAPITAL LETTER NU
pub const FCITX_KEY_Greek_NU: u32 = 0x07cd;

/// U+039E GREEK CAPITAL LETTER XI
pub const FCITX_KEY_Greek_XI: u32 = 0x07ce;

/// U+039F GREEK CAPITAL LETTER OMICRON
pub const FCITX_KEY_Greek_OMICRON: u32 = 0x07cf;

/// U+03A0 GREEK CAPITAL LETTER PI
pub const FCITX_KEY_Greek_PI: u32 = 0x07d0;

/// U+03A1 GREEK CAPITAL LETTER RHO
pub const FCITX_KEY_Greek_RHO: u32 = 0x07d1;

/// U+03A3 GREEK CAPITAL LETTER SIGMA
pub const FCITX_KEY_Greek_SIGMA: u32 = 0x07d2;

/// U+03A4 GREEK CAPITAL LETTER TAU
pub const FCITX_KEY_Greek_TAU: u32 = 0x07d4;

/// U+03A5 GREEK CAPITAL LETTER UPSILON
pub const FCITX_KEY_Greek_UPSILON: u32 = 0x07d5;

/// U+03A6 GREEK CAPITAL LETTER PHI
pub const FCITX_KEY_Greek_PHI: u32 = 0x07d6;

/// U+03A7 GREEK CAPITAL LETTER CHI
pub const FCITX_KEY_Greek_CHI: u32 = 0x07d7;

/// U+03A8 GREEK CAPITAL LETTER PSI
pub const FCITX_KEY_Greek_PSI: u32 = 0x07d8;

/// U+03A9 GREEK CAPITAL LETTER OMEGA
pub const FCITX_KEY_Greek_OMEGA: u32 = 0x07d9;

/// U+03B1 GREEK SMALL LETTER ALPHA
pub const FCITX_KEY_Greek_alpha: u32 = 0x07e1;

/// U+03B2 GREEK SMALL LETTER BETA
pub const FCITX_KEY_Greek_beta: u32 = 0x07e2;

/// U+03B3 GREEK SMALL LETTER GAMMA
pub const FCITX_KEY_Greek_gamma: u32 = 0x07e3;

/// U+03B4 GREEK SMALL LETTER DELTA
pub const FCITX_KEY_Greek_delta: u32 = 0x07e4;

/// U+03B5 GREEK SMALL LETTER EPSILON
pub const FCITX_KEY_Greek_epsilon: u32 = 0x07e5;

/// U+03B6 GREEK SMALL LETTER ZETA
pub const FCITX_KEY_Greek_zeta: u32 = 0x07e6;

/// U+03B7 GREEK SMALL LETTER ETA
pub const FCITX_KEY_Greek_eta: u32 = 0x07e7;

/// U+03B8 GREEK SMALL LETTER THETA
pub const FCITX_KEY_Greek_theta: u32 = 0x07e8;

/// U+03B9 GREEK SMALL LETTER IOTA
pub const FCITX_KEY_Greek_iota: u32 = 0x07e9;

/// U+03BA GREEK SMALL LETTER KAPPA
pub const FCITX_KEY_Greek_kappa: u32 = 0x07ea;

/// U+03BB GREEK SMALL LETTER LAMDA
pub const FCITX_KEY_Greek_lamda: u32 = 0x07eb;

/// non-deprecated alias for Greek_lamda
pub const FCITX_KEY_Greek_lambda: u32 = 0x07eb;

/// U+03BC GREEK SMALL LETTER MU
pub const FCITX_KEY_Greek_mu: u32 = 0x07ec;

/// U+03BD GREEK SMALL LETTER NU
pub const FCITX_KEY_Greek_nu: u32 = 0x07ed;

/// U+03BE GREEK SMALL LETTER XI
pub const FCITX_KEY_Greek_xi: u32 = 0x07ee;

/// U+03BF GREEK SMALL LETTER OMICRON
pub const FCITX_KEY_Greek_omicron: u32 = 0x07ef;

/// U+03C0 GREEK SMALL LETTER PI
pub const FCITX_KEY_Greek_pi: u32 = 0x07f0;

/// U+03C1 GREEK SMALL LETTER RHO
pub const FCITX_KEY_Greek_rho: u32 = 0x07f1;

/// U+03C3 GREEK SMALL LETTER SIGMA
pub const FCITX_KEY_Greek_sigma: u32 = 0x07f2;

/// U+03C2 GREEK SMALL LETTER FINAL SIGMA
pub const FCITX_KEY_Greek_finalsmallsigma: u32 = 0x07f3;

/// U+03C4 GREEK SMALL LETTER TAU
pub const FCITX_KEY_Greek_tau: u32 = 0x07f4;

/// U+03C5 GREEK SMALL LETTER UPSILON
pub const FCITX_KEY_Greek_upsilon: u32 = 0x07f5;

/// U+03C6 GREEK SMALL LETTER PHI
pub const FCITX_KEY_Greek_phi: u32 = 0x07f6;

/// U+03C7 GREEK SMALL LETTER CHI
pub const FCITX_KEY_Greek_chi: u32 = 0x07f7;

/// U+03C8 GREEK SMALL LETTER PSI
pub const FCITX_KEY_Greek_psi: u32 = 0x07f8;

/// U+03C9 GREEK SMALL LETTER OMEGA
pub const FCITX_KEY_Greek_omega: u32 = 0x07f9;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_Greek_switch: u32 = 0xff7e;

/// U+23B7 RADICAL SYMBOL BOTTOM
pub const FCITX_KEY_leftradical: u32 = 0x08a1;

/// (U+250C BOX DRAWINGS LIGHT DOWN AND RIGHT)
pub const FCITX_KEY_topleftradical: u32 = 0x08a2;

/// (U+2500 BOX DRAWINGS LIGHT HORIZONTAL)
pub const FCITX_KEY_horizconnector: u32 = 0x08a3;

/// U+2320 TOP HALF INTEGRAL
pub const FCITX_KEY_topintegral: u32 = 0x08a4;

/// U+2321 BOTTOM HALF INTEGRAL
pub const FCITX_KEY_botintegral: u32 = 0x08a5;

/// (U+2502 BOX DRAWINGS LIGHT VERTICAL)
pub const FCITX_KEY_vertconnector: u32 = 0x08a6;

/// U+23A1 LEFT SQUARE BRACKET UPPER CORNER
pub const FCITX_KEY_topleftsqbracket: u32 = 0x08a7;

/// U+23A3 LEFT SQUARE BRACKET LOWER CORNER
pub const FCITX_KEY_botleftsqbracket: u32 = 0x08a8;

/// U+23A4 RIGHT SQUARE BRACKET UPPER CORNER
pub const FCITX_KEY_toprightsqbracket: u32 = 0x08a9;

/// U+23A6 RIGHT SQUARE BRACKET LOWER CORNER
pub const FCITX_KEY_botrightsqbracket: u32 = 0x08aa;

/// U+239B LEFT PARENTHESIS UPPER HOOK
pub const FCITX_KEY_topleftparens: u32 = 0x08ab;

/// U+239D LEFT PARENTHESIS LOWER HOOK
pub const FCITX_KEY_botleftparens: u32 = 0x08ac;

/// U+239E RIGHT PARENTHESIS UPPER HOOK
pub const FCITX_KEY_toprightparens: u32 = 0x08ad;

/// U+23A0 RIGHT PARENTHESIS LOWER HOOK
pub const FCITX_KEY_botrightparens: u32 = 0x08ae;

/// U+23A8 LEFT CURLY BRACKET MIDDLE PIECE
pub const FCITX_KEY_leftmiddlecurlybrace: u32 = 0x08af;

/// U+23AC RIGHT CURLY BRACKET MIDDLE PIECE
pub const FCITX_KEY_rightmiddlecurlybrace: u32 = 0x08b0;

pub const FCITX_KEY_topleftsummation: u32 = 0x08b1;

pub const FCITX_KEY_botleftsummation: u32 = 0x08b2;

pub const FCITX_KEY_topvertsummationconnector: u32 = 0x08b3;

pub const FCITX_KEY_botvertsummationconnector: u32 = 0x08b4;

pub const FCITX_KEY_toprightsummation: u32 = 0x08b5;

pub const FCITX_KEY_botrightsummation: u32 = 0x08b6;

pub const FCITX_KEY_rightmiddlesummation: u32 = 0x08b7;

/// U+2264 LESS-THAN OR EQUAL TO
pub const FCITX_KEY_lessthanequal: u32 = 0x08bc;

/// U+2260 NOT EQUAL TO
pub const FCITX_KEY_notequal: u32 = 0x08bd;

/// U+2265 GREATER-THAN OR EQUAL TO
pub const FCITX_KEY_greaterthanequal: u32 = 0x08be;

/// U+222B INTEGRAL
pub const FCITX_KEY_integral: u32 = 0x08bf;

/// U+2234 THEREFORE
pub const FCITX_KEY_therefore: u32 = 0x08c0;

/// U+221D PROPORTIONAL TO
pub const FCITX_KEY_variation: u32 = 0x08c1;

/// U+221E INFINITY
pub const FCITX_KEY_infinity: u32 = 0x08c2;

/// U+2207 NABLA
pub const FCITX_KEY_nabla: u32 = 0x08c5;

/// U+223C TILDE OPERATOR
pub const FCITX_KEY_approximate: u32 = 0x08c8;

/// U+2243 ASYMPTOTICALLY EQUAL TO
pub const FCITX_KEY_similarequal: u32 = 0x08c9;

/// U+21D4 LEFT RIGHT DOUBLE ARROW
pub const FCITX_KEY_ifonlyif: u32 = 0x08cd;

/// U+21D2 RIGHTWARDS DOUBLE ARROW
pub const FCITX_KEY_implies: u32 = 0x08ce;

/// U+2261 IDENTICAL TO
pub const FCITX_KEY_identical: u32 = 0x08cf;

/// U+221A SQUARE ROOT
pub const FCITX_KEY_radical: u32 = 0x08d6;

/// U+2282 SUBSET OF
pub const FCITX_KEY_includedin: u32 = 0x08da;

/// U+2283 SUPERSET OF
pub const FCITX_KEY_includes: u32 = 0x08db;

/// U+2229 INTERSECTION
pub const FCITX_KEY_intersection: u32 = 0x08dc;

/// U+222A UNION
pub const FCITX_KEY_union: u32 = 0x08dd;

/// U+2227 LOGICAL AND
pub const FCITX_KEY_logicaland: u32 = 0x08de;

/// U+2228 LOGICAL OR
pub const FCITX_KEY_logicalor: u32 = 0x08df;

/// U+2202 PARTIAL DIFFERENTIAL
pub const FCITX_KEY_partialderivative: u32 = 0x08ef;

/// U+0192 LATIN SMALL LETTER F WITH HOOK
pub const FCITX_KEY_function: u32 = 0x08f6;

/// U+2190 LEFTWARDS ARROW
pub const FCITX_KEY_leftarrow: u32 = 0x08fb;

/// U+2191 UPWARDS ARROW
pub const FCITX_KEY_uparrow: u32 = 0x08fc;

/// U+2192 RIGHTWARDS ARROW
pub const FCITX_KEY_rightarrow: u32 = 0x08fd;

/// U+2193 DOWNWARDS ARROW
pub const FCITX_KEY_downarrow: u32 = 0x08fe;

pub const FCITX_KEY_blank: u32 = 0x09df;

/// U+25C6 BLACK DIAMOND
pub const FCITX_KEY_soliddiamond: u32 = 0x09e0;

/// U+2592 MEDIUM SHADE
pub const FCITX_KEY_checkerboard: u32 = 0x09e1;

/// U+2409 SYMBOL FOR HORIZONTAL TABULATION
pub const FCITX_KEY_ht: u32 = 0x09e2;

/// U+240C SYMBOL FOR FORM FEED
pub const FCITX_KEY_ff: u32 = 0x09e3;

/// U+240D SYMBOL FOR CARRIAGE RETURN
pub const FCITX_KEY_cr: u32 = 0x09e4;

/// U+240A SYMBOL FOR LINE FEED
pub const FCITX_KEY_lf: u32 = 0x09e5;

/// U+2424 SYMBOL FOR NEWLINE
pub const FCITX_KEY_nl: u32 = 0x09e8;

/// U+240B SYMBOL FOR VERTICAL TABULATION
pub const FCITX_KEY_vt: u32 = 0x09e9;

/// U+2518 BOX DRAWINGS LIGHT UP AND LEFT
pub const FCITX_KEY_lowrightcorner: u32 = 0x09ea;

/// U+2510 BOX DRAWINGS LIGHT DOWN AND LEFT
pub const FCITX_KEY_uprightcorner: u32 = 0x09eb;

/// U+250C BOX DRAWINGS LIGHT DOWN AND RIGHT
pub const FCITX_KEY_upleftcorner: u32 = 0x09ec;

/// U+2514 BOX DRAWINGS LIGHT UP AND RIGHT
pub const FCITX_KEY_lowleftcorner: u32 = 0x09ed;

/// U+253C BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
pub const FCITX_KEY_crossinglines: u32 = 0x09ee;

/// U+23BA HORIZONTAL SCAN LINE-1
pub const FCITX_KEY_horizlinescan1: u32 = 0x09ef;

/// U+23BB HORIZONTAL SCAN LINE-3
pub const FCITX_KEY_horizlinescan3: u32 = 0x09f0;

/// U+2500 BOX DRAWINGS LIGHT HORIZONTAL
pub const FCITX_KEY_horizlinescan5: u32 = 0x09f1;

/// U+23BC HORIZONTAL SCAN LINE-7
pub const FCITX_KEY_horizlinescan7: u32 = 0x09f2;

/// U+23BD HORIZONTAL SCAN LINE-9
pub const FCITX_KEY_horizlinescan9: u32 = 0x09f3;

/// U+251C BOX DRAWINGS LIGHT VERTICAL AND RIGHT
pub const FCITX_KEY_leftt: u32 = 0x09f4;

/// U+2524 BOX DRAWINGS LIGHT VERTICAL AND LEFT
pub const FCITX_KEY_rightt: u32 = 0x09f5;

/// U+2534 BOX DRAWINGS LIGHT UP AND HORIZONTAL
pub const FCITX_KEY_bott: u32 = 0x09f6;

/// U+252C BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
pub const FCITX_KEY_topt: u32 = 0x09f7;

/// U+2502 BOX DRAWINGS LIGHT VERTICAL
pub const FCITX_KEY_vertbar: u32 = 0x09f8;

/// U+2003 EM SPACE
pub const FCITX_KEY_emspace: u32 = 0x0aa1;

/// U+2002 EN SPACE
pub const FCITX_KEY_enspace: u32 = 0x0aa2;

/// U+2004 THREE-PER-EM SPACE
pub const FCITX_KEY_em3space: u32 = 0x0aa3;

/// U+2005 FOUR-PER-EM SPACE
pub const FCITX_KEY_em4space: u32 = 0x0aa4;

/// U+2007 FIGURE SPACE
pub const FCITX_KEY_digitspace: u32 = 0x0aa5;

/// U+2008 PUNCTUATION SPACE
pub const FCITX_KEY_punctspace: u32 = 0x0aa6;

/// U+2009 THIN SPACE
pub const FCITX_KEY_thinspace: u32 = 0x0aa7;

/// U+200A HAIR SPACE
pub const FCITX_KEY_hairspace: u32 = 0x0aa8;

/// U+2014 EM DASH
pub const FCITX_KEY_emdash: u32 = 0x0aa9;

/// U+2013 EN DASH
pub const FCITX_KEY_endash: u32 = 0x0aaa;

/// (U+2423 OPEN BOX)
pub const FCITX_KEY_signifblank: u32 = 0x0aac;

/// U+2026 HORIZONTAL ELLIPSIS
pub const FCITX_KEY_ellipsis: u32 = 0x0aae;

/// U+2025 TWO DOT LEADER
pub const FCITX_KEY_doubbaselinedot: u32 = 0x0aaf;

/// U+2153 VULGAR FRACTION ONE THIRD
pub const FCITX_KEY_onethird: u32 = 0x0ab0;

/// U+2154 VULGAR FRACTION TWO THIRDS
pub const FCITX_KEY_twothirds: u32 = 0x0ab1;

/// U+2155 VULGAR FRACTION ONE FIFTH
pub const FCITX_KEY_onefifth: u32 = 0x0ab2;

/// U+2156 VULGAR FRACTION TWO FIFTHS
pub const FCITX_KEY_twofifths: u32 = 0x0ab3;

/// U+2157 VULGAR FRACTION THREE FIFTHS
pub const FCITX_KEY_threefifths: u32 = 0x0ab4;

/// U+2158 VULGAR FRACTION FOUR FIFTHS
pub const FCITX_KEY_fourfifths: u32 = 0x0ab5;

/// U+2159 VULGAR FRACTION ONE SIXTH
pub const FCITX_KEY_onesixth: u32 = 0x0ab6;

/// U+215A VULGAR FRACTION FIVE SIXTHS
pub const FCITX_KEY_fivesixths: u32 = 0x0ab7;

/// U+2105 CARE OF
pub const FCITX_KEY_careof: u32 = 0x0ab8;

/// U+2012 FIGURE DASH
pub const FCITX_KEY_figdash: u32 = 0x0abb;

/// (U+2329 LEFT-POINTING ANGLE BRACKET)
pub const FCITX_KEY_leftanglebracket: u32 = 0x0abc;

/// (U+002E FULL STOP)
pub const FCITX_KEY_decimalpoint: u32 = 0x0abd;

/// (U+232A RIGHT-POINTING ANGLE BRACKET)
pub const FCITX_KEY_rightanglebracket: u32 = 0x0abe;

pub const FCITX_KEY_marker: u32 = 0x0abf;

/// U+215B VULGAR FRACTION ONE EIGHTH
pub const FCITX_KEY_oneeighth: u32 = 0x0ac3;

/// U+215C VULGAR FRACTION THREE EIGHTHS
pub const FCITX_KEY_threeeighths: u32 = 0x0ac4;

/// U+215D VULGAR FRACTION FIVE EIGHTHS
pub const FCITX_KEY_fiveeighths: u32 = 0x0ac5;

/// U+215E VULGAR FRACTION SEVEN EIGHTHS
pub const FCITX_KEY_seveneighths: u32 = 0x0ac6;

/// U+2122 TRADE MARK SIGN
pub const FCITX_KEY_trademark: u32 = 0x0ac9;

/// (U+2613 SALTIRE)
pub const FCITX_KEY_signaturemark: u32 = 0x0aca;

pub const FCITX_KEY_trademarkincircle: u32 = 0x0acb;

/// (U+25C1 WHITE LEFT-POINTING TRIANGLE)
pub const FCITX_KEY_leftopentriangle: u32 = 0x0acc;

/// (U+25B7 WHITE RIGHT-POINTING TRIANGLE)
pub const FCITX_KEY_rightopentriangle: u32 = 0x0acd;

/// (U+25CB WHITE CIRCLE)
pub const FCITX_KEY_emopencircle: u32 = 0x0ace;

/// (U+25AF WHITE VERTICAL RECTANGLE)
pub const FCITX_KEY_emopenrectangle: u32 = 0x0acf;

/// U+2018 LEFT SINGLE QUOTATION MARK
pub const FCITX_KEY_leftsinglequotemark: u32 = 0x0ad0;

/// U+2019 RIGHT SINGLE QUOTATION MARK
pub const FCITX_KEY_rightsinglequotemark: u32 = 0x0ad1;

/// U+201C LEFT DOUBLE QUOTATION MARK
pub const FCITX_KEY_leftdoublequotemark: u32 = 0x0ad2;

/// U+201D RIGHT DOUBLE QUOTATION MARK
pub const FCITX_KEY_rightdoublequotemark: u32 = 0x0ad3;

/// U+211E PRESCRIPTION TAKE
pub const FCITX_KEY_prescription: u32 = 0x0ad4;

/// U+2030 PER MILLE SIGN
pub const FCITX_KEY_permille: u32 = 0x0ad5;

/// U+2032 PRIME
pub const FCITX_KEY_minutes: u32 = 0x0ad6;

/// U+2033 DOUBLE PRIME
pub const FCITX_KEY_seconds: u32 = 0x0ad7;

/// U+271D LATIN CROSS
pub const FCITX_KEY_latincross: u32 = 0x0ad9;

pub const FCITX_KEY_hexagram: u32 = 0x0ada;

/// (U+25AC BLACK RECTANGLE)
pub const FCITX_KEY_filledrectbullet: u32 = 0x0adb;

/// (U+25C0 BLACK LEFT-POINTING TRIANGLE)
pub const FCITX_KEY_filledlefttribullet: u32 = 0x0adc;

/// (U+25B6 BLACK RIGHT-POINTING TRIANGLE)
pub const FCITX_KEY_filledrighttribullet: u32 = 0x0add;

/// (U+25CF BLACK CIRCLE)
pub const FCITX_KEY_emfilledcircle: u32 = 0x0ade;

/// (U+25AE BLACK VERTICAL RECTANGLE)
pub const FCITX_KEY_emfilledrect: u32 = 0x0adf;

/// (U+25E6 WHITE BULLET)
pub const FCITX_KEY_enopencircbullet: u32 = 0x0ae0;

/// (U+25AB WHITE SMALL SQUARE)
pub const FCITX_KEY_enopensquarebullet: u32 = 0x0ae1;

/// (U+25AD WHITE RECTANGLE)
pub const FCITX_KEY_openrectbullet: u32 = 0x0ae2;

/// (U+25B3 WHITE UP-POINTING TRIANGLE)
pub const FCITX_KEY_opentribulletup: u32 = 0x0ae3;

/// (U+25BD WHITE DOWN-POINTING TRIANGLE)
pub const FCITX_KEY_opentribulletdown: u32 = 0x0ae4;

/// (U+2606 WHITE STAR)
pub const FCITX_KEY_openstar: u32 = 0x0ae5;

/// (U+2022 BULLET)
pub const FCITX_KEY_enfilledcircbullet: u32 = 0x0ae6;

/// (U+25AA BLACK SMALL SQUARE)
pub const FCITX_KEY_enfilledsqbullet: u32 = 0x0ae7;

/// (U+25B2 BLACK UP-POINTING TRIANGLE)
pub const FCITX_KEY_filledtribulletup: u32 = 0x0ae8;

/// (U+25BC BLACK DOWN-POINTING TRIANGLE)
pub const FCITX_KEY_filledtribulletdown: u32 = 0x0ae9;

/// (U+261C WHITE LEFT POINTING INDEX)
pub const FCITX_KEY_leftpointer: u32 = 0x0aea;

/// (U+261E WHITE RIGHT POINTING INDEX)
pub const FCITX_KEY_rightpointer: u32 = 0x0aeb;

/// U+2663 BLACK CLUB SUIT
pub const FCITX_KEY_club: u32 = 0x0aec;

/// U+2666 BLACK DIAMOND SUIT
pub const FCITX_KEY_diamond: u32 = 0x0aed;

/// U+2665 BLACK HEART SUIT
pub const FCITX_KEY_heart: u32 = 0x0aee;

/// U+2720 MALTESE CROSS
pub const FCITX_KEY_maltesecross: u32 = 0x0af0;

/// U+2020 DAGGER
pub const FCITX_KEY_dagger: u32 = 0x0af1;

/// U+2021 DOUBLE DAGGER
pub const FCITX_KEY_doubledagger: u32 = 0x0af2;

/// U+2713 CHECK MARK
pub const FCITX_KEY_checkmark: u32 = 0x0af3;

/// U+2717 BALLOT X
pub const FCITX_KEY_ballotcross: u32 = 0x0af4;

/// U+266F MUSIC SHARP SIGN
pub const FCITX_KEY_musicalsharp: u32 = 0x0af5;

/// U+266D MUSIC FLAT SIGN
pub const FCITX_KEY_musicalflat: u32 = 0x0af6;

/// U+2642 MALE SIGN
pub const FCITX_KEY_malesymbol: u32 = 0x0af7;

/// U+2640 FEMALE SIGN
pub const FCITX_KEY_femalesymbol: u32 = 0x0af8;

/// U+260E BLACK TELEPHONE
pub const FCITX_KEY_telephone: u32 = 0x0af9;

/// U+2315 TELEPHONE RECORDER
pub const FCITX_KEY_telephonerecorder: u32 = 0x0afa;

/// U+2117 SOUND RECORDING COPYRIGHT
pub const FCITX_KEY_phonographcopyright: u32 = 0x0afb;

/// U+2038 CARET
pub const FCITX_KEY_caret: u32 = 0x0afc;

/// U+201A SINGLE LOW-9 QUOTATION MARK
pub const FCITX_KEY_singlelowquotemark: u32 = 0x0afd;

/// U+201E DOUBLE LOW-9 QUOTATION MARK
pub const FCITX_KEY_doublelowquotemark: u32 = 0x0afe;

pub const FCITX_KEY_cursor: u32 = 0x0aff;

/// (U+003C LESS-THAN SIGN)
pub const FCITX_KEY_leftcaret: u32 = 0x0ba3;

/// (U+003E GREATER-THAN SIGN)
pub const FCITX_KEY_rightcaret: u32 = 0x0ba6;

/// (U+2228 LOGICAL OR)
pub const FCITX_KEY_downcaret: u32 = 0x0ba8;

/// (U+2227 LOGICAL AND)
pub const FCITX_KEY_upcaret: u32 = 0x0ba9;

/// (U+00AF MACRON)
pub const FCITX_KEY_overbar: u32 = 0x0bc0;

/// U+22A4 DOWN TACK
pub const FCITX_KEY_downtack: u32 = 0x0bc2;

/// (U+2229 INTERSECTION)
pub const FCITX_KEY_upshoe: u32 = 0x0bc3;

/// U+230A LEFT FLOOR
pub const FCITX_KEY_downstile: u32 = 0x0bc4;

/// (U+005F LOW LINE)
pub const FCITX_KEY_underbar: u32 = 0x0bc6;

/// U+2218 RING OPERATOR
pub const FCITX_KEY_jot: u32 = 0x0bca;

/// U+2395 APL FUNCTIONAL SYMBOL QUAD
pub const FCITX_KEY_quad: u32 = 0x0bcc;

/// U+22A5 UP TACK
pub const FCITX_KEY_uptack: u32 = 0x0bce;

/// U+25CB WHITE CIRCLE
pub const FCITX_KEY_circle: u32 = 0x0bcf;

/// U+2308 LEFT CEILING
pub const FCITX_KEY_upstile: u32 = 0x0bd3;

/// (U+222A UNION)
pub const FCITX_KEY_downshoe: u32 = 0x0bd6;

/// (U+2283 SUPERSET OF)
pub const FCITX_KEY_rightshoe: u32 = 0x0bd8;

/// (U+2282 SUBSET OF)
pub const FCITX_KEY_leftshoe: u32 = 0x0bda;

/// U+22A3 LEFT TACK
pub const FCITX_KEY_lefttack: u32 = 0x0bdc;

/// U+22A2 RIGHT TACK
pub const FCITX_KEY_righttack: u32 = 0x0bfc;

/// U+2017 DOUBLE LOW LINE
pub const FCITX_KEY_hebrew_doublelowline: u32 = 0x0cdf;

/// U+05D0 HEBREW LETTER ALEF
pub const FCITX_KEY_hebrew_aleph: u32 = 0x0ce0;

/// U+05D1 HEBREW LETTER BET
pub const FCITX_KEY_hebrew_bet: u32 = 0x0ce1;

/// deprecated
pub const FCITX_KEY_hebrew_beth: u32 = 0x0ce1;

/// U+05D2 HEBREW LETTER GIMEL
pub const FCITX_KEY_hebrew_gimel: u32 = 0x0ce2;

/// deprecated
pub const FCITX_KEY_hebrew_gimmel: u32 = 0x0ce2;

/// U+05D3 HEBREW LETTER DALET
pub const FCITX_KEY_hebrew_dalet: u32 = 0x0ce3;

/// deprecated
pub const FCITX_KEY_hebrew_daleth: u32 = 0x0ce3;

/// U+05D4 HEBREW LETTER HE
pub const FCITX_KEY_hebrew_he: u32 = 0x0ce4;

/// U+05D5 HEBREW LETTER VAV
pub const FCITX_KEY_hebrew_waw: u32 = 0x0ce5;

/// U+05D6 HEBREW LETTER ZAYIN
pub const FCITX_KEY_hebrew_zain: u32 = 0x0ce6;

/// deprecated
pub const FCITX_KEY_hebrew_zayin: u32 = 0x0ce6;

/// U+05D7 HEBREW LETTER HET
pub const FCITX_KEY_hebrew_chet: u32 = 0x0ce7;

/// deprecated
pub const FCITX_KEY_hebrew_het: u32 = 0x0ce7;

/// U+05D8 HEBREW LETTER TET
pub const FCITX_KEY_hebrew_tet: u32 = 0x0ce8;

/// deprecated
pub const FCITX_KEY_hebrew_teth: u32 = 0x0ce8;

/// U+05D9 HEBREW LETTER YOD
pub const FCITX_KEY_hebrew_yod: u32 = 0x0ce9;

/// U+05DA HEBREW LETTER FINAL KAF
pub const FCITX_KEY_hebrew_finalkaph: u32 = 0x0cea;

/// U+05DB HEBREW LETTER KAF
pub const FCITX_KEY_hebrew_kaph: u32 = 0x0ceb;

/// U+05DC HEBREW LETTER LAMED
pub const FCITX_KEY_hebrew_lamed: u32 = 0x0cec;

/// U+05DD HEBREW LETTER FINAL MEM
pub const FCITX_KEY_hebrew_finalmem: u32 = 0x0ced;

/// U+05DE HEBREW LETTER MEM
pub const FCITX_KEY_hebrew_mem: u32 = 0x0cee;

/// U+05DF HEBREW LETTER FINAL NUN
pub const FCITX_KEY_hebrew_finalnun: u32 = 0x0cef;

/// U+05E0 HEBREW LETTER NUN
pub const FCITX_KEY_hebrew_nun: u32 = 0x0cf0;

/// U+05E1 HEBREW LETTER SAMEKH
pub const FCITX_KEY_hebrew_samech: u32 = 0x0cf1;

/// deprecated
pub const FCITX_KEY_hebrew_samekh: u32 = 0x0cf1;

/// U+05E2 HEBREW LETTER AYIN
pub const FCITX_KEY_hebrew_ayin: u32 = 0x0cf2;

/// U+05E3 HEBREW LETTER FINAL PE
pub const FCITX_KEY_hebrew_finalpe: u32 = 0x0cf3;

/// U+05E4 HEBREW LETTER PE
pub const FCITX_KEY_hebrew_pe: u32 = 0x0cf4;

/// U+05E5 HEBREW LETTER FINAL TSADI
pub const FCITX_KEY_hebrew_finalzade: u32 = 0x0cf5;

/// deprecated
pub const FCITX_KEY_hebrew_finalzadi: u32 = 0x0cf5;

/// U+05E6 HEBREW LETTER TSADI
pub const FCITX_KEY_hebrew_zade: u32 = 0x0cf6;

/// deprecated
pub const FCITX_KEY_hebrew_zadi: u32 = 0x0cf6;

/// U+05E7 HEBREW LETTER QOF
pub const FCITX_KEY_hebrew_qoph: u32 = 0x0cf7;

/// deprecated
pub const FCITX_KEY_hebrew_kuf: u32 = 0x0cf7;

/// U+05E8 HEBREW LETTER RESH
pub const FCITX_KEY_hebrew_resh: u32 = 0x0cf8;

/// U+05E9 HEBREW LETTER SHIN
pub const FCITX_KEY_hebrew_shin: u32 = 0x0cf9;

/// U+05EA HEBREW LETTER TAV
pub const FCITX_KEY_hebrew_taw: u32 = 0x0cfa;

/// deprecated
pub const FCITX_KEY_hebrew_taf: u32 = 0x0cfa;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_Hebrew_switch: u32 = 0xff7e;

/// U+0E01 THAI CHARACTER KO KAI
pub const FCITX_KEY_Thai_kokai: u32 = 0x0da1;

/// U+0E02 THAI CHARACTER KHO KHAI
pub const FCITX_KEY_Thai_khokhai: u32 = 0x0da2;

/// U+0E03 THAI CHARACTER KHO KHUAT
pub const FCITX_KEY_Thai_khokhuat: u32 = 0x0da3;

/// U+0E04 THAI CHARACTER KHO KHWAI
pub const FCITX_KEY_Thai_khokhwai: u32 = 0x0da4;

/// U+0E05 THAI CHARACTER KHO KHON
pub const FCITX_KEY_Thai_khokhon: u32 = 0x0da5;

/// U+0E06 THAI CHARACTER KHO RAKHANG
pub const FCITX_KEY_Thai_khorakhang: u32 = 0x0da6;

/// U+0E07 THAI CHARACTER NGO NGU
pub const FCITX_KEY_Thai_ngongu: u32 = 0x0da7;

/// U+0E08 THAI CHARACTER CHO CHAN
pub const FCITX_KEY_Thai_chochan: u32 = 0x0da8;

/// U+0E09 THAI CHARACTER CHO CHING
pub const FCITX_KEY_Thai_choching: u32 = 0x0da9;

/// U+0E0A THAI CHARACTER CHO CHANG
pub const FCITX_KEY_Thai_chochang: u32 = 0x0daa;

/// U+0E0B THAI CHARACTER SO SO
pub const FCITX_KEY_Thai_soso: u32 = 0x0dab;

/// U+0E0C THAI CHARACTER CHO CHOE
pub const FCITX_KEY_Thai_chochoe: u32 = 0x0dac;

/// U+0E0D THAI CHARACTER YO YING
pub const FCITX_KEY_Thai_yoying: u32 = 0x0dad;

/// U+0E0E THAI CHARACTER DO CHADA
pub const FCITX_KEY_Thai_dochada: u32 = 0x0dae;

/// U+0E0F THAI CHARACTER TO PATAK
pub const FCITX_KEY_Thai_topatak: u32 = 0x0daf;

/// U+0E10 THAI CHARACTER THO THAN
pub const FCITX_KEY_Thai_thothan: u32 = 0x0db0;

/// U+0E11 THAI CHARACTER THO NANGMONTHO
pub const FCITX_KEY_Thai_thonangmontho: u32 = 0x0db1;

/// U+0E12 THAI CHARACTER THO PHUTHAO
pub const FCITX_KEY_Thai_thophuthao: u32 = 0x0db2;

/// U+0E13 THAI CHARACTER NO NEN
pub const FCITX_KEY_Thai_nonen: u32 = 0x0db3;

/// U+0E14 THAI CHARACTER DO DEK
pub const FCITX_KEY_Thai_dodek: u32 = 0x0db4;

/// U+0E15 THAI CHARACTER TO TAO
pub const FCITX_KEY_Thai_totao: u32 = 0x0db5;

/// U+0E16 THAI CHARACTER THO THUNG
pub const FCITX_KEY_Thai_thothung: u32 = 0x0db6;

/// U+0E17 THAI CHARACTER THO THAHAN
pub const FCITX_KEY_Thai_thothahan: u32 = 0x0db7;

/// U+0E18 THAI CHARACTER THO THONG
pub const FCITX_KEY_Thai_thothong: u32 = 0x0db8;

/// U+0E19 THAI CHARACTER NO NU
pub const FCITX_KEY_Thai_nonu: u32 = 0x0db9;

/// U+0E1A THAI CHARACTER BO BAIMAI
pub const FCITX_KEY_Thai_bobaimai: u32 = 0x0dba;

/// U+0E1B THAI CHARACTER PO PLA
pub const FCITX_KEY_Thai_popla: u32 = 0x0dbb;

/// U+0E1C THAI CHARACTER PHO PHUNG
pub const FCITX_KEY_Thai_phophung: u32 = 0x0dbc;

/// U+0E1D THAI CHARACTER FO FA
pub const FCITX_KEY_Thai_fofa: u32 = 0x0dbd;

/// U+0E1E THAI CHARACTER PHO PHAN
pub const FCITX_KEY_Thai_phophan: u32 = 0x0dbe;

/// U+0E1F THAI CHARACTER FO FAN
pub const FCITX_KEY_Thai_fofan: u32 = 0x0dbf;

/// U+0E20 THAI CHARACTER PHO SAMPHAO
pub const FCITX_KEY_Thai_phosamphao: u32 = 0x0dc0;

/// U+0E21 THAI CHARACTER MO MA
pub const FCITX_KEY_Thai_moma: u32 = 0x0dc1;

/// U+0E22 THAI CHARACTER YO YAK
pub const FCITX_KEY_Thai_yoyak: u32 = 0x0dc2;

/// U+0E23 THAI CHARACTER RO RUA
pub const FCITX_KEY_Thai_rorua: u32 = 0x0dc3;

/// U+0E24 THAI CHARACTER RU
pub const FCITX_KEY_Thai_ru: u32 = 0x0dc4;

/// U+0E25 THAI CHARACTER LO LING
pub const FCITX_KEY_Thai_loling: u32 = 0x0dc5;

/// U+0E26 THAI CHARACTER LU
pub const FCITX_KEY_Thai_lu: u32 = 0x0dc6;

/// U+0E27 THAI CHARACTER WO WAEN
pub const FCITX_KEY_Thai_wowaen: u32 = 0x0dc7;

/// U+0E28 THAI CHARACTER SO SALA
pub const FCITX_KEY_Thai_sosala: u32 = 0x0dc8;

/// U+0E29 THAI CHARACTER SO RUSI
pub const FCITX_KEY_Thai_sorusi: u32 = 0x0dc9;

/// U+0E2A THAI CHARACTER SO SUA
pub const FCITX_KEY_Thai_sosua: u32 = 0x0dca;

/// U+0E2B THAI CHARACTER HO HIP
pub const FCITX_KEY_Thai_hohip: u32 = 0x0dcb;

/// U+0E2C THAI CHARACTER LO CHULA
pub const FCITX_KEY_Thai_lochula: u32 = 0x0dcc;

/// U+0E2D THAI CHARACTER O ANG
pub const FCITX_KEY_Thai_oang: u32 = 0x0dcd;

/// U+0E2E THAI CHARACTER HO NOKHUK
pub const FCITX_KEY_Thai_honokhuk: u32 = 0x0dce;

/// U+0E2F THAI CHARACTER PAIYANNOI
pub const FCITX_KEY_Thai_paiyannoi: u32 = 0x0dcf;

/// U+0E30 THAI CHARACTER SARA A
pub const FCITX_KEY_Thai_saraa: u32 = 0x0dd0;

/// U+0E31 THAI CHARACTER MAI HAN-AKAT
pub const FCITX_KEY_Thai_maihanakat: u32 = 0x0dd1;

/// U+0E32 THAI CHARACTER SARA AA
pub const FCITX_KEY_Thai_saraaa: u32 = 0x0dd2;

/// U+0E33 THAI CHARACTER SARA AM
pub const FCITX_KEY_Thai_saraam: u32 = 0x0dd3;

/// U+0E34 THAI CHARACTER SARA I
pub const FCITX_KEY_Thai_sarai: u32 = 0x0dd4;

/// U+0E35 THAI CHARACTER SARA II
pub const FCITX_KEY_Thai_saraii: u32 = 0x0dd5;

/// U+0E36 THAI CHARACTER SARA UE
pub const FCITX_KEY_Thai_saraue: u32 = 0x0dd6;

/// U+0E37 THAI CHARACTER SARA UEE
pub const FCITX_KEY_Thai_sarauee: u32 = 0x0dd7;

/// U+0E38 THAI CHARACTER SARA U
pub const FCITX_KEY_Thai_sarau: u32 = 0x0dd8;

/// U+0E39 THAI CHARACTER SARA UU
pub const FCITX_KEY_Thai_sarauu: u32 = 0x0dd9;

/// U+0E3A THAI CHARACTER PHINTHU
pub const FCITX_KEY_Thai_phinthu: u32 = 0x0dda;

/// (U+0E3E Unassigned code point)
pub const FCITX_KEY_Thai_maihanakat_maitho: u32 = 0x0dde;

/// U+0E3F THAI CURRENCY SYMBOL BAHT
pub const FCITX_KEY_Thai_baht: u32 = 0x0ddf;

/// U+0E40 THAI CHARACTER SARA E
pub const FCITX_KEY_Thai_sarae: u32 = 0x0de0;

/// U+0E41 THAI CHARACTER SARA AE
pub const FCITX_KEY_Thai_saraae: u32 = 0x0de1;

/// U+0E42 THAI CHARACTER SARA O
pub const FCITX_KEY_Thai_sarao: u32 = 0x0de2;

/// U+0E43 THAI CHARACTER SARA AI MAIMUAN
pub const FCITX_KEY_Thai_saraaimaimuan: u32 = 0x0de3;

/// U+0E44 THAI CHARACTER SARA AI MAIMALAI
pub const FCITX_KEY_Thai_saraaimaimalai: u32 = 0x0de4;

/// U+0E45 THAI CHARACTER LAKKHANGYAO
pub const FCITX_KEY_Thai_lakkhangyao: u32 = 0x0de5;

/// U+0E46 THAI CHARACTER MAIYAMOK
pub const FCITX_KEY_Thai_maiyamok: u32 = 0x0de6;

/// U+0E47 THAI CHARACTER MAITAIKHU
pub const FCITX_KEY_Thai_maitaikhu: u32 = 0x0de7;

/// U+0E48 THAI CHARACTER MAI EK
pub const FCITX_KEY_Thai_maiek: u32 = 0x0de8;

/// U+0E49 THAI CHARACTER MAI THO
pub const FCITX_KEY_Thai_maitho: u32 = 0x0de9;

/// U+0E4A THAI CHARACTER MAI TRI
pub const FCITX_KEY_Thai_maitri: u32 = 0x0dea;

/// U+0E4B THAI CHARACTER MAI CHATTAWA
pub const FCITX_KEY_Thai_maichattawa: u32 = 0x0deb;

/// U+0E4C THAI CHARACTER THANTHAKHAT
pub const FCITX_KEY_Thai_thanthakhat: u32 = 0x0dec;

/// U+0E4D THAI CHARACTER NIKHAHIT
pub const FCITX_KEY_Thai_nikhahit: u32 = 0x0ded;

/// U+0E50 THAI DIGIT ZERO
pub const FCITX_KEY_Thai_leksun: u32 = 0x0df0;

/// U+0E51 THAI DIGIT ONE
pub const FCITX_KEY_Thai_leknung: u32 = 0x0df1;

/// U+0E52 THAI DIGIT TWO
pub const FCITX_KEY_Thai_leksong: u32 = 0x0df2;

/// U+0E53 THAI DIGIT THREE
pub const FCITX_KEY_Thai_leksam: u32 = 0x0df3;

/// U+0E54 THAI DIGIT FOUR
pub const FCITX_KEY_Thai_leksi: u32 = 0x0df4;

/// U+0E55 THAI DIGIT FIVE
pub const FCITX_KEY_Thai_lekha: u32 = 0x0df5;

/// U+0E56 THAI DIGIT SIX
pub const FCITX_KEY_Thai_lekhok: u32 = 0x0df6;

/// U+0E57 THAI DIGIT SEVEN
pub const FCITX_KEY_Thai_lekchet: u32 = 0x0df7;

/// U+0E58 THAI DIGIT EIGHT
pub const FCITX_KEY_Thai_lekpaet: u32 = 0x0df8;

/// U+0E59 THAI DIGIT NINE
pub const FCITX_KEY_Thai_lekkao: u32 = 0x0df9;

/// Hangul start/stop(toggle)
pub const FCITX_KEY_Hangul: u32 = 0xff31;

/// Hangul start
pub const FCITX_KEY_Hangul_Start: u32 = 0xff32;

/// Hangul end, English start
pub const FCITX_KEY_Hangul_End: u32 = 0xff33;

/// Start Hangul->Hanja Conversion
pub const FCITX_KEY_Hangul_Hanja: u32 = 0xff34;

/// Hangul Jamo mode
pub const FCITX_KEY_Hangul_Jamo: u32 = 0xff35;

/// Hangul Romaja mode
pub const FCITX_KEY_Hangul_Romaja: u32 = 0xff36;

/// Hangul code input mode
pub const FCITX_KEY_Hangul_Codeinput: u32 = 0xff37;

/// Jeonja mode
pub const FCITX_KEY_Hangul_Jeonja: u32 = 0xff38;

/// Banja mode
pub const FCITX_KEY_Hangul_Banja: u32 = 0xff39;

/// Pre Hanja conversion
pub const FCITX_KEY_Hangul_PreHanja: u32 = 0xff3a;

/// Post Hanja conversion
pub const FCITX_KEY_Hangul_PostHanja: u32 = 0xff3b;

/// Single candidate
pub const FCITX_KEY_Hangul_SingleCandidate: u32 = 0xff3c;

/// Multiple candidate
pub const FCITX_KEY_Hangul_MultipleCandidate: u32 = 0xff3d;

/// Previous candidate
pub const FCITX_KEY_Hangul_PreviousCandidate: u32 = 0xff3e;

/// Special symbols
pub const FCITX_KEY_Hangul_Special: u32 = 0xff3f;

/// non-deprecated alias for Mode_switch
pub const FCITX_KEY_Hangul_switch: u32 = 0xff7e;

/// U+3131 HANGUL LETTER KIYEOK
pub const FCITX_KEY_Hangul_Kiyeog: u32 = 0x0ea1;

/// U+3132 HANGUL LETTER SSANGKIYEOK
pub const FCITX_KEY_Hangul_SsangKiyeog: u32 = 0x0ea2;

/// U+3133 HANGUL LETTER KIYEOK-SIOS
pub const FCITX_KEY_Hangul_KiyeogSios: u32 = 0x0ea3;

/// U+3134 HANGUL LETTER NIEUN
pub const FCITX_KEY_Hangul_Nieun: u32 = 0x0ea4;

/// U+3135 HANGUL LETTER NIEUN-CIEUC
pub const FCITX_KEY_Hangul_NieunJieuj: u32 = 0x0ea5;

/// U+3136 HANGUL LETTER NIEUN-HIEUH
pub const FCITX_KEY_Hangul_NieunHieuh: u32 = 0x0ea6;

/// U+3137 HANGUL LETTER TIKEUT
pub const FCITX_KEY_Hangul_Dikeud: u32 = 0x0ea7;

/// U+3138 HANGUL LETTER SSANGTIKEUT
pub const FCITX_KEY_Hangul_SsangDikeud: u32 = 0x0ea8;

/// U+3139 HANGUL LETTER RIEUL
pub const FCITX_KEY_Hangul_Rieul: u32 = 0x0ea9;

/// U+313A HANGUL LETTER RIEUL-KIYEOK
pub const FCITX_KEY_Hangul_RieulKiyeog: u32 = 0x0eaa;

/// U+313B HANGUL LETTER RIEUL-MIEUM
pub const FCITX_KEY_Hangul_RieulMieum: u32 = 0x0eab;

/// U+313C HANGUL LETTER RIEUL-PIEUP
pub const FCITX_KEY_Hangul_RieulPieub: u32 = 0x0eac;

/// U+313D HANGUL LETTER RIEUL-SIOS
pub const FCITX_KEY_Hangul_RieulSios: u32 = 0x0ead;

/// U+313E HANGUL LETTER RIEUL-THIEUTH
pub const FCITX_KEY_Hangul_RieulTieut: u32 = 0x0eae;

/// U+313F HANGUL LETTER RIEUL-PHIEUPH
pub const FCITX_KEY_Hangul_RieulPhieuf: u32 = 0x0eaf;

/// U+3140 HANGUL LETTER RIEUL-HIEUH
pub const FCITX_KEY_Hangul_RieulHieuh: u32 = 0x0eb0;

/// U+3141 HANGUL LETTER MIEUM
pub const FCITX_KEY_Hangul_Mieum: u32 = 0x0eb1;

/// U+3142 HANGUL LETTER PIEUP
pub const FCITX_KEY_Hangul_Pieub: u32 = 0x0eb2;

/// U+3143 HANGUL LETTER SSANGPIEUP
pub const FCITX_KEY_Hangul_SsangPieub: u32 = 0x0eb3;

/// U+3144 HANGUL LETTER PIEUP-SIOS
pub const FCITX_KEY_Hangul_PieubSios: u32 = 0x0eb4;

/// U+3145 HANGUL LETTER SIOS
pub const FCITX_KEY_Hangul_Sios: u32 = 0x0eb5;

/// U+3146 HANGUL LETTER SSANGSIOS
pub const FCITX_KEY_Hangul_SsangSios: u32 = 0x0eb6;

/// U+3147 HANGUL LETTER IEUNG
pub const FCITX_KEY_Hangul_Ieung: u32 = 0x0eb7;

/// U+3148 HANGUL LETTER CIEUC
pub const FCITX_KEY_Hangul_Jieuj: u32 = 0x0eb8;

/// U+3149 HANGUL LETTER SSANGCIEUC
pub const FCITX_KEY_Hangul_SsangJieuj: u32 = 0x0eb9;

/// U+314A HANGUL LETTER CHIEUCH
pub const FCITX_KEY_Hangul_Cieuc: u32 = 0x0eba;

/// U+314B HANGUL LETTER KHIEUKH
pub const FCITX_KEY_Hangul_Khieuq: u32 = 0x0ebb;

/// U+314C HANGUL LETTER THIEUTH
pub const FCITX_KEY_Hangul_Tieut: u32 = 0x0ebc;

/// U+314D HANGUL LETTER PHIEUPH
pub const FCITX_KEY_Hangul_Phieuf: u32 = 0x0ebd;

/// U+314E HANGUL LETTER HIEUH
pub const FCITX_KEY_Hangul_Hieuh: u32 = 0x0ebe;

/// U+314F HANGUL LETTER A
pub const FCITX_KEY_Hangul_A: u32 = 0x0ebf;

/// U+3150 HANGUL LETTER AE
pub const FCITX_KEY_Hangul_AE: u32 = 0x0ec0;

/// U+3151 HANGUL LETTER YA
pub const FCITX_KEY_Hangul_YA: u32 = 0x0ec1;

/// U+3152 HANGUL LETTER YAE
pub const FCITX_KEY_Hangul_YAE: u32 = 0x0ec2;

/// U+3153 HANGUL LETTER EO
pub const FCITX_KEY_Hangul_EO: u32 = 0x0ec3;

/// U+3154 HANGUL LETTER E
pub const FCITX_KEY_Hangul_E: u32 = 0x0ec4;

/// U+3155 HANGUL LETTER YEO
pub const FCITX_KEY_Hangul_YEO: u32 = 0x0ec5;

/// U+3156 HANGUL LETTER YE
pub const FCITX_KEY_Hangul_YE: u32 = 0x0ec6;

/// U+3157 HANGUL LETTER O
pub const FCITX_KEY_Hangul_O: u32 = 0x0ec7;

/// U+3158 HANGUL LETTER WA
pub const FCITX_KEY_Hangul_WA: u32 = 0x0ec8;

/// U+3159 HANGUL LETTER WAE
pub const FCITX_KEY_Hangul_WAE: u32 = 0x0ec9;

/// U+315A HANGUL LETTER OE
pub const FCITX_KEY_Hangul_OE: u32 = 0x0eca;

/// U+315B HANGUL LETTER YO
pub const FCITX_KEY_Hangul_YO: u32 = 0x0ecb;

/// U+315C HANGUL LETTER U
pub const FCITX_KEY_Hangul_U: u32 = 0x0ecc;

/// U+315D HANGUL LETTER WEO
pub const FCITX_KEY_Hangul_WEO: u32 = 0x0ecd;

/// U+315E HANGUL LETTER WE
pub const FCITX_KEY_Hangul_WE: u32 = 0x0ece;

/// U+315F HANGUL LETTER WI
pub const FCITX_KEY_Hangul_WI: u32 = 0x0ecf;

/// U+3160 HANGUL LETTER YU
pub const FCITX_KEY_Hangul_YU: u32 = 0x0ed0;

/// U+3161 HANGUL LETTER EU
pub const FCITX_KEY_Hangul_EU: u32 = 0x0ed1;

/// U+3162 HANGUL LETTER YI
pub const FCITX_KEY_Hangul_YI: u32 = 0x0ed2;

/// U+3163 HANGUL LETTER I
pub const FCITX_KEY_Hangul_I: u32 = 0x0ed3;

/// U+11A8 HANGUL JONGSEONG KIYEOK
pub const FCITX_KEY_Hangul_J_Kiyeog: u32 = 0x0ed4;

/// U+11A9 HANGUL JONGSEONG SSANGKIYEOK
pub const FCITX_KEY_Hangul_J_SsangKiyeog: u32 = 0x0ed5;

/// U+11AA HANGUL JONGSEONG KIYEOK-SIOS
pub const FCITX_KEY_Hangul_J_KiyeogSios: u32 = 0x0ed6;

/// U+11AB HANGUL JONGSEONG NIEUN
pub const FCITX_KEY_Hangul_J_Nieun: u32 = 0x0ed7;

/// U+11AC HANGUL JONGSEONG NIEUN-CIEUC
pub const FCITX_KEY_Hangul_J_NieunJieuj: u32 = 0x0ed8;

/// U+11AD HANGUL JONGSEONG NIEUN-HIEUH
pub const FCITX_KEY_Hangul_J_NieunHieuh: u32 = 0x0ed9;

/// U+11AE HANGUL JONGSEONG TIKEUT
pub const FCITX_KEY_Hangul_J_Dikeud: u32 = 0x0eda;

/// U+11AF HANGUL JONGSEONG RIEUL
pub const FCITX_KEY_Hangul_J_Rieul: u32 = 0x0edb;

/// U+11B0 HANGUL JONGSEONG RIEUL-KIYEOK
pub const FCITX_KEY_Hangul_J_RieulKiyeog: u32 = 0x0edc;

/// U+11B1 HANGUL JONGSEONG RIEUL-MIEUM
pub const FCITX_KEY_Hangul_J_RieulMieum: u32 = 0x0edd;

/// U+11B2 HANGUL JONGSEONG RIEUL-PIEUP
pub const FCITX_KEY_Hangul_J_RieulPieub: u32 = 0x0ede;

/// U+11B3 HANGUL JONGSEONG RIEUL-SIOS
pub const FCITX_KEY_Hangul_J_RieulSios: u32 = 0x0edf;

/// U+11B4 HANGUL JONGSEONG RIEUL-THIEUTH
pub const FCITX_KEY_Hangul_J_RieulTieut: u32 = 0x0ee0;

/// U+11B5 HANGUL JONGSEONG RIEUL-PHIEUPH
pub const FCITX_KEY_Hangul_J_RieulPhieuf: u32 = 0x0ee1;

/// U+11B6 HANGUL JONGSEONG RIEUL-HIEUH
pub const FCITX_KEY_Hangul_J_RieulHieuh: u32 = 0x0ee2;

/// U+11B7 HANGUL JONGSEONG MIEUM
pub const FCITX_KEY_Hangul_J_Mieum: u32 = 0x0ee3;

/// U+11B8 HANGUL JONGSEONG PIEUP
pub const FCITX_KEY_Hangul_J_Pieub: u32 = 0x0ee4;

/// U+11B9 HANGUL JONGSEONG PIEUP-SIOS
pub const FCITX_KEY_Hangul_J_PieubSios: u32 = 0x0ee5;

/// U+11BA HANGUL JONGSEONG SIOS
pub const FCITX_KEY_Hangul_J_Sios: u32 = 0x0ee6;

/// U+11BB HANGUL JONGSEONG SSANGSIOS
pub const FCITX_KEY_Hangul_J_SsangSios: u32 = 0x0ee7;

/// U+11BC HANGUL JONGSEONG IEUNG
pub const FCITX_KEY_Hangul_J_Ieung: u32 = 0x0ee8;

/// U+11BD HANGUL JONGSEONG CIEUC
pub const FCITX_KEY_Hangul_J_Jieuj: u32 = 0x0ee9;

/// U+11BE HANGUL JONGSEONG CHIEUCH
pub const FCITX_KEY_Hangul_J_Cieuc: u32 = 0x0eea;

/// U+11BF HANGUL JONGSEONG KHIEUKH
pub const FCITX_KEY_Hangul_J_Khieuq: u32 = 0x0eeb;

/// U+11C0 HANGUL JONGSEONG THIEUTH
pub const FCITX_KEY_Hangul_J_Tieut: u32 = 0x0eec;

/// U+11C1 HANGUL JONGSEONG PHIEUPH
pub const FCITX_KEY_Hangul_J_Phieuf: u32 = 0x0eed;

/// U+11C2 HANGUL JONGSEONG HIEUH
pub const FCITX_KEY_Hangul_J_Hieuh: u32 = 0x0eee;

/// U+316D HANGUL LETTER RIEUL-YEORINHIEUH
pub const FCITX_KEY_Hangul_RieulYeorinHieuh: u32 = 0x0eef;

/// U+3171 HANGUL LETTER KAPYEOUNMIEUM
pub const FCITX_KEY_Hangul_SunkyeongeumMieum: u32 = 0x0ef0;

/// U+3178 HANGUL LETTER KAPYEOUNPIEUP
pub const FCITX_KEY_Hangul_SunkyeongeumPieub: u32 = 0x0ef1;

/// U+317F HANGUL LETTER PANSIOS
pub const FCITX_KEY_Hangul_PanSios: u32 = 0x0ef2;

/// U+3181 HANGUL LETTER YESIEUNG
pub const FCITX_KEY_Hangul_KkogjiDalrinIeung: u32 = 0x0ef3;

/// U+3184 HANGUL LETTER KAPYEOUNPHIEUPH
pub const FCITX_KEY_Hangul_SunkyeongeumPhieuf: u32 = 0x0ef4;

/// U+3186 HANGUL LETTER YEORINHIEUH
pub const FCITX_KEY_Hangul_YeorinHieuh: u32 = 0x0ef5;

/// U+318D HANGUL LETTER ARAEA
pub const FCITX_KEY_Hangul_AraeA: u32 = 0x0ef6;

/// U+318E HANGUL LETTER ARAEAE
pub const FCITX_KEY_Hangul_AraeAE: u32 = 0x0ef7;

/// U+11EB HANGUL JONGSEONG PANSIOS
pub const FCITX_KEY_Hangul_J_PanSios: u32 = 0x0ef8;

/// U+11F0 HANGUL JONGSEONG YESIEUNG
pub const FCITX_KEY_Hangul_J_KkogjiDalrinIeung: u32 = 0x0ef9;

/// U+11F9 HANGUL JONGSEONG YEORINHIEUH
pub const FCITX_KEY_Hangul_J_YeorinHieuh: u32 = 0x0efa;

/// (U+20A9 WON SIGN)
pub const FCITX_KEY_Korean_Won: u32 = 0x0eff;

/// U+0587 ARMENIAN SMALL LIGATURE ECH YIWN
pub const FCITX_KEY_Armenian_ligature_ew: u32 = 0x1000587;

/// U+0589 ARMENIAN FULL STOP
pub const FCITX_KEY_Armenian_full_stop: u32 = 0x1000589;

/// deprecated alias for Armenian_full_stop
pub const FCITX_KEY_Armenian_verjaket: u32 = 0x1000589;

/// U+055D ARMENIAN COMMA
pub const FCITX_KEY_Armenian_separation_mark: u32 = 0x100055d;

/// deprecated alias for Armenian_separation_mark
pub const FCITX_KEY_Armenian_but: u32 = 0x100055d;

/// U+058A ARMENIAN HYPHEN
pub const FCITX_KEY_Armenian_hyphen: u32 = 0x100058a;

/// deprecated alias for Armenian_hyphen
pub const FCITX_KEY_Armenian_yentamna: u32 = 0x100058a;

/// U+055C ARMENIAN EXCLAMATION MARK
pub const FCITX_KEY_Armenian_exclam: u32 = 0x100055c;

/// deprecated alias for Armenian_exclam
pub const FCITX_KEY_Armenian_amanak: u32 = 0x100055c;

/// U+055B ARMENIAN EMPHASIS MARK
pub const FCITX_KEY_Armenian_accent: u32 = 0x100055b;

/// deprecated alias for Armenian_accent
pub const FCITX_KEY_Armenian_shesht: u32 = 0x100055b;

/// U+055E ARMENIAN QUESTION MARK
pub const FCITX_KEY_Armenian_question: u32 = 0x100055e;

/// deprecated alias for Armenian_question
pub const FCITX_KEY_Armenian_paruyk: u32 = 0x100055e;

/// U+0531 ARMENIAN CAPITAL LETTER AYB
pub const FCITX_KEY_Armenian_AYB: u32 = 0x1000531;

/// U+0561 ARMENIAN SMALL LETTER AYB
pub const FCITX_KEY_Armenian_ayb: u32 = 0x1000561;

/// U+0532 ARMENIAN CAPITAL LETTER BEN
pub const FCITX_KEY_Armenian_BEN: u32 = 0x1000532;

/// U+0562 ARMENIAN SMALL LETTER BEN
pub const FCITX_KEY_Armenian_ben: u32 = 0x1000562;

/// U+0533 ARMENIAN CAPITAL LETTER GIM
pub const FCITX_KEY_Armenian_GIM: u32 = 0x1000533;

/// U+0563 ARMENIAN SMALL LETTER GIM
pub const FCITX_KEY_Armenian_gim: u32 = 0x1000563;

/// U+0534 ARMENIAN CAPITAL LETTER DA
pub const FCITX_KEY_Armenian_DA: u32 = 0x1000534;

/// U+0564 ARMENIAN SMALL LETTER DA
pub const FCITX_KEY_Armenian_da: u32 = 0x1000564;

/// U+0535 ARMENIAN CAPITAL LETTER ECH
pub const FCITX_KEY_Armenian_YECH: u32 = 0x1000535;

/// U+0565 ARMENIAN SMALL LETTER ECH
pub const FCITX_KEY_Armenian_yech: u32 = 0x1000565;

/// U+0536 ARMENIAN CAPITAL LETTER ZA
pub const FCITX_KEY_Armenian_ZA: u32 = 0x1000536;

/// U+0566 ARMENIAN SMALL LETTER ZA
pub const FCITX_KEY_Armenian_za: u32 = 0x1000566;

/// U+0537 ARMENIAN CAPITAL LETTER EH
pub const FCITX_KEY_Armenian_E: u32 = 0x1000537;

/// U+0567 ARMENIAN SMALL LETTER EH
pub const FCITX_KEY_Armenian_e: u32 = 0x1000567;

/// U+0538 ARMENIAN CAPITAL LETTER ET
pub const FCITX_KEY_Armenian_AT: u32 = 0x1000538;

/// U+0568 ARMENIAN SMALL LETTER ET
pub const FCITX_KEY_Armenian_at: u32 = 0x1000568;

/// U+0539 ARMENIAN CAPITAL LETTER TO
pub const FCITX_KEY_Armenian_TO: u32 = 0x1000539;

/// U+0569 ARMENIAN SMALL LETTER TO
pub const FCITX_KEY_Armenian_to: u32 = 0x1000569;

/// U+053A ARMENIAN CAPITAL LETTER ZHE
pub const FCITX_KEY_Armenian_ZHE: u32 = 0x100053a;

/// U+056A ARMENIAN SMALL LETTER ZHE
pub const FCITX_KEY_Armenian_zhe: u32 = 0x100056a;

/// U+053B ARMENIAN CAPITAL LETTER INI
pub const FCITX_KEY_Armenian_INI: u32 = 0x100053b;

/// U+056B ARMENIAN SMALL LETTER INI
pub const FCITX_KEY_Armenian_ini: u32 = 0x100056b;

/// U+053C ARMENIAN CAPITAL LETTER LIWN
pub const FCITX_KEY_Armenian_LYUN: u32 = 0x100053c;

/// U+056C ARMENIAN SMALL LETTER LIWN
pub const FCITX_KEY_Armenian_lyun: u32 = 0x100056c;

/// U+053D ARMENIAN CAPITAL LETTER XEH
pub const FCITX_KEY_Armenian_KHE: u32 = 0x100053d;

/// U+056D ARMENIAN SMALL LETTER XEH
pub const FCITX_KEY_Armenian_khe: u32 = 0x100056d;

/// U+053E ARMENIAN CAPITAL LETTER CA
pub const FCITX_KEY_Armenian_TSA: u32 = 0x100053e;

/// U+056E ARMENIAN SMALL LETTER CA
pub const FCITX_KEY_Armenian_tsa: u32 = 0x100056e;

/// U+053F ARMENIAN CAPITAL LETTER KEN
pub const FCITX_KEY_Armenian_KEN: u32 = 0x100053f;

/// U+056F ARMENIAN SMALL LETTER KEN
pub const FCITX_KEY_Armenian_ken: u32 = 0x100056f;

/// U+0540 ARMENIAN CAPITAL LETTER HO
pub const FCITX_KEY_Armenian_HO: u32 = 0x1000540;

/// U+0570 ARMENIAN SMALL LETTER HO
pub const FCITX_KEY_Armenian_ho: u32 = 0x1000570;

/// U+0541 ARMENIAN CAPITAL LETTER JA
pub const FCITX_KEY_Armenian_DZA: u32 = 0x1000541;

/// U+0571 ARMENIAN SMALL LETTER JA
pub const FCITX_KEY_Armenian_dza: u32 = 0x1000571;

/// U+0542 ARMENIAN CAPITAL LETTER GHAD
pub const FCITX_KEY_Armenian_GHAT: u32 = 0x1000542;

/// U+0572 ARMENIAN SMALL LETTER GHAD
pub const FCITX_KEY_Armenian_ghat: u32 = 0x1000572;

/// U+0543 ARMENIAN CAPITAL LETTER CHEH
pub const FCITX_KEY_Armenian_TCHE: u32 = 0x1000543;

/// U+0573 ARMENIAN SMALL LETTER CHEH
pub const FCITX_KEY_Armenian_tche: u32 = 0x1000573;

/// U+0544 ARMENIAN CAPITAL LETTER MEN
pub const FCITX_KEY_Armenian_MEN: u32 = 0x1000544;

/// U+0574 ARMENIAN SMALL LETTER MEN
pub const FCITX_KEY_Armenian_men: u32 = 0x1000574;

/// U+0545 ARMENIAN CAPITAL LETTER YI
pub const FCITX_KEY_Armenian_HI: u32 = 0x1000545;

/// U+0575 ARMENIAN SMALL LETTER YI
pub const FCITX_KEY_Armenian_hi: u32 = 0x1000575;

/// U+0546 ARMENIAN CAPITAL LETTER NOW
pub const FCITX_KEY_Armenian_NU: u32 = 0x1000546;

/// U+0576 ARMENIAN SMALL LETTER NOW
pub const FCITX_KEY_Armenian_nu: u32 = 0x1000576;

/// U+0547 ARMENIAN CAPITAL LETTER SHA
pub const FCITX_KEY_Armenian_SHA: u32 = 0x1000547;

/// U+0577 ARMENIAN SMALL LETTER SHA
pub const FCITX_KEY_Armenian_sha: u32 = 0x1000577;

/// U+0548 ARMENIAN CAPITAL LETTER VO
pub const FCITX_KEY_Armenian_VO: u32 = 0x1000548;

/// U+0578 ARMENIAN SMALL LETTER VO
pub const FCITX_KEY_Armenian_vo: u32 = 0x1000578;

/// U+0549 ARMENIAN CAPITAL LETTER CHA
pub const FCITX_KEY_Armenian_CHA: u32 = 0x1000549;

/// U+0579 ARMENIAN SMALL LETTER CHA
pub const FCITX_KEY_Armenian_cha: u32 = 0x1000579;

/// U+054A ARMENIAN CAPITAL LETTER PEH
pub const FCITX_KEY_Armenian_PE: u32 = 0x100054a;

/// U+057A ARMENIAN SMALL LETTER PEH
pub const FCITX_KEY_Armenian_pe: u32 = 0x100057a;

/// U+054B ARMENIAN CAPITAL LETTER JHEH
pub const FCITX_KEY_Armenian_JE: u32 = 0x100054b;

/// U+057B ARMENIAN SMALL LETTER JHEH
pub const FCITX_KEY_Armenian_je: u32 = 0x100057b;

/// U+054C ARMENIAN CAPITAL LETTER RA
pub const FCITX_KEY_Armenian_RA: u32 = 0x100054c;

/// U+057C ARMENIAN SMALL LETTER RA
pub const FCITX_KEY_Armenian_ra: u32 = 0x100057c;

/// U+054D ARMENIAN CAPITAL LETTER SEH
pub const FCITX_KEY_Armenian_SE: u32 = 0x100054d;

/// U+057D ARMENIAN SMALL LETTER SEH
pub const FCITX_KEY_Armenian_se: u32 = 0x100057d;

/// U+054E ARMENIAN CAPITAL LETTER VEW
pub const FCITX_KEY_Armenian_VEV: u32 = 0x100054e;

/// U+057E ARMENIAN SMALL LETTER VEW
pub const FCITX_KEY_Armenian_vev: u32 = 0x100057e;

/// U+054F ARMENIAN CAPITAL LETTER TIWN
pub const FCITX_KEY_Armenian_TYUN: u32 = 0x100054f;

/// U+057F ARMENIAN SMALL LETTER TIWN
pub const FCITX_KEY_Armenian_tyun: u32 = 0x100057f;

/// U+0550 ARMENIAN CAPITAL LETTER REH
pub const FCITX_KEY_Armenian_RE: u32 = 0x1000550;

/// U+0580 ARMENIAN SMALL LETTER REH
pub const FCITX_KEY_Armenian_re: u32 = 0x1000580;

/// U+0551 ARMENIAN CAPITAL LETTER CO
pub const FCITX_KEY_Armenian_TSO: u32 = 0x1000551;

/// U+0581 ARMENIAN SMALL LETTER CO
pub const FCITX_KEY_Armenian_tso: u32 = 0x1000581;

/// U+0552 ARMENIAN CAPITAL LETTER YIWN
pub const FCITX_KEY_Armenian_VYUN: u32 = 0x1000552;

/// U+0582 ARMENIAN SMALL LETTER YIWN
pub const FCITX_KEY_Armenian_vyun: u32 = 0x1000582;

/// U+0553 ARMENIAN CAPITAL LETTER PIWR
pub const FCITX_KEY_Armenian_PYUR: u32 = 0x1000553;

/// U+0583 ARMENIAN SMALL LETTER PIWR
pub const FCITX_KEY_Armenian_pyur: u32 = 0x1000583;

/// U+0554 ARMENIAN CAPITAL LETTER KEH
pub const FCITX_KEY_Armenian_KE: u32 = 0x1000554;

/// U+0584 ARMENIAN SMALL LETTER KEH
pub const FCITX_KEY_Armenian_ke: u32 = 0x1000584;

/// U+0555 ARMENIAN CAPITAL LETTER OH
pub const FCITX_KEY_Armenian_O: u32 = 0x1000555;

/// U+0585 ARMENIAN SMALL LETTER OH
pub const FCITX_KEY_Armenian_o: u32 = 0x1000585;

/// U+0556 ARMENIAN CAPITAL LETTER FEH
pub const FCITX_KEY_Armenian_FE: u32 = 0x1000556;

/// U+0586 ARMENIAN SMALL LETTER FEH
pub const FCITX_KEY_Armenian_fe: u32 = 0x1000586;

/// U+055A ARMENIAN APOSTROPHE
pub const FCITX_KEY_Armenian_apostrophe: u32 = 0x100055a;

/// U+10D0 GEORGIAN LETTER AN
pub const FCITX_KEY_Georgian_an: u32 = 0x10010d0;

/// U+10D1 GEORGIAN LETTER BAN
pub const FCITX_KEY_Georgian_ban: u32 = 0x10010d1;

/// U+10D2 GEORGIAN LETTER GAN
pub const FCITX_KEY_Georgian_gan: u32 = 0x10010d2;

/// U+10D3 GEORGIAN LETTER DON
pub const FCITX_KEY_Georgian_don: u32 = 0x10010d3;

/// U+10D4 GEORGIAN LETTER EN
pub const FCITX_KEY_Georgian_en: u32 = 0x10010d4;

/// U+10D5 GEORGIAN LETTER VIN
pub const FCITX_KEY_Georgian_vin: u32 = 0x10010d5;

/// U+10D6 GEORGIAN LETTER ZEN
pub const FCITX_KEY_Georgian_zen: u32 = 0x10010d6;

/// U+10D7 GEORGIAN LETTER TAN
pub const FCITX_KEY_Georgian_tan: u32 = 0x10010d7;

/// U+10D8 GEORGIAN LETTER IN
pub const FCITX_KEY_Georgian_in: u32 = 0x10010d8;

/// U+10D9 GEORGIAN LETTER KAN
pub const FCITX_KEY_Georgian_kan: u32 = 0x10010d9;

/// U+10DA GEORGIAN LETTER LAS
pub const FCITX_KEY_Georgian_las: u32 = 0x10010da;

/// U+10DB GEORGIAN LETTER MAN
pub const FCITX_KEY_Georgian_man: u32 = 0x10010db;

/// U+10DC GEORGIAN LETTER NAR
pub const FCITX_KEY_Georgian_nar: u32 = 0x10010dc;

/// U+10DD GEORGIAN LETTER ON
pub const FCITX_KEY_Georgian_on: u32 = 0x10010dd;

/// U+10DE GEORGIAN LETTER PAR
pub const FCITX_KEY_Georgian_par: u32 = 0x10010de;

/// U+10DF GEORGIAN LETTER ZHAR
pub const FCITX_KEY_Georgian_zhar: u32 = 0x10010df;

/// U+10E0 GEORGIAN LETTER RAE
pub const FCITX_KEY_Georgian_rae: u32 = 0x10010e0;

/// U+10E1 GEORGIAN LETTER SAN
pub const FCITX_KEY_Georgian_san: u32 = 0x10010e1;

/// U+10E2 GEORGIAN LETTER TAR
pub const FCITX_KEY_Georgian_tar: u32 = 0x10010e2;

/// U+10E3 GEORGIAN LETTER UN
pub const FCITX_KEY_Georgian_un: u32 = 0x10010e3;

/// U+10E4 GEORGIAN LETTER PHAR
pub const FCITX_KEY_Georgian_phar: u32 = 0x10010e4;

/// U+10E5 GEORGIAN LETTER KHAR
pub const FCITX_KEY_Georgian_khar: u32 = 0x10010e5;

/// U+10E6 GEORGIAN LETTER GHAN
pub const FCITX_KEY_Georgian_ghan: u32 = 0x10010e6;

/// U+10E7 GEORGIAN LETTER QAR
pub const FCITX_KEY_Georgian_qar: u32 = 0x10010e7;

/// U+10E8 GEORGIAN LETTER SHIN
pub const FCITX_KEY_Georgian_shin: u32 = 0x10010e8;

/// U+10E9 GEORGIAN LETTER CHIN
pub const FCITX_KEY_Georgian_chin: u32 = 0x10010e9;

/// U+10EA GEORGIAN LETTER CAN
pub const FCITX_KEY_Georgian_can: u32 = 0x10010ea;

/// U+10EB GEORGIAN LETTER JIL
pub const FCITX_KEY_Georgian_jil: u32 = 0x10010eb;

/// U+10EC GEORGIAN LETTER CIL
pub const FCITX_KEY_Georgian_cil: u32 = 0x10010ec;

/// U+10ED GEORGIAN LETTER CHAR
pub const FCITX_KEY_Georgian_char: u32 = 0x10010ed;

/// U+10EE GEORGIAN LETTER XAN
pub const FCITX_KEY_Georgian_xan: u32 = 0x10010ee;

/// U+10EF GEORGIAN LETTER JHAN
pub const FCITX_KEY_Georgian_jhan: u32 = 0x10010ef;

/// U+10F0 GEORGIAN LETTER HAE
pub const FCITX_KEY_Georgian_hae: u32 = 0x10010f0;

/// U+10F1 GEORGIAN LETTER HE
pub const FCITX_KEY_Georgian_he: u32 = 0x10010f1;

/// U+10F2 GEORGIAN LETTER HIE
pub const FCITX_KEY_Georgian_hie: u32 = 0x10010f2;

/// U+10F3 GEORGIAN LETTER WE
pub const FCITX_KEY_Georgian_we: u32 = 0x10010f3;

/// U+10F4 GEORGIAN LETTER HAR
pub const FCITX_KEY_Georgian_har: u32 = 0x10010f4;

/// U+10F5 GEORGIAN LETTER HOE
pub const FCITX_KEY_Georgian_hoe: u32 = 0x10010f5;

/// U+10F6 GEORGIAN LETTER FI
pub const FCITX_KEY_Georgian_fi: u32 = 0x10010f6;

/// U+1E8A LATIN CAPITAL LETTER X WITH DOT ABOVE
pub const FCITX_KEY_Xabovedot: u32 = 0x1001e8a;

/// U+012C LATIN CAPITAL LETTER I WITH BREVE
pub const FCITX_KEY_Ibreve: u32 = 0x100012c;

/// U+01B5 LATIN CAPITAL LETTER Z WITH STROKE
pub const FCITX_KEY_Zstroke: u32 = 0x10001b5;

/// U+01E6 LATIN CAPITAL LETTER G WITH CARON
pub const FCITX_KEY_Gcaron: u32 = 0x10001e6;

/// U+01D1 LATIN CAPITAL LETTER O WITH CARON
pub const FCITX_KEY_Ocaron: u32 = 0x10001d1;

/// U+019F LATIN CAPITAL LETTER O WITH MIDDLE TILDE
pub const FCITX_KEY_Obarred: u32 = 0x100019f;

/// U+1E8B LATIN SMALL LETTER X WITH DOT ABOVE
pub const FCITX_KEY_xabovedot: u32 = 0x1001e8b;

/// U+012D LATIN SMALL LETTER I WITH BREVE
pub const FCITX_KEY_ibreve: u32 = 0x100012d;

/// U+01B6 LATIN SMALL LETTER Z WITH STROKE
pub const FCITX_KEY_zstroke: u32 = 0x10001b6;

/// U+01E7 LATIN SMALL LETTER G WITH CARON
pub const FCITX_KEY_gcaron: u32 = 0x10001e7;

/// U+01D2 LATIN SMALL LETTER O WITH CARON
pub const FCITX_KEY_ocaron: u32 = 0x10001d2;

/// U+0275 LATIN SMALL LETTER BARRED O
pub const FCITX_KEY_obarred: u32 = 0x1000275;

/// U+018F LATIN CAPITAL LETTER SCHWA
pub const FCITX_KEY_SCHWA: u32 = 0x100018f;

/// U+0259 LATIN SMALL LETTER SCHWA
pub const FCITX_KEY_schwa: u32 = 0x1000259;

/// U+01B7 LATIN CAPITAL LETTER EZH
pub const FCITX_KEY_EZH: u32 = 0x10001b7;

/// U+0292 LATIN SMALL LETTER EZH
pub const FCITX_KEY_ezh: u32 = 0x1000292;

/// U+1E36 LATIN CAPITAL LETTER L WITH DOT BELOW
pub const FCITX_KEY_Lbelowdot: u32 = 0x1001e36;

/// U+1E37 LATIN SMALL LETTER L WITH DOT BELOW
pub const FCITX_KEY_lbelowdot: u32 = 0x1001e37;

/// U+1EA0 LATIN CAPITAL LETTER A WITH DOT BELOW
pub const FCITX_KEY_Abelowdot: u32 = 0x1001ea0;

/// U+1EA1 LATIN SMALL LETTER A WITH DOT BELOW
pub const FCITX_KEY_abelowdot: u32 = 0x1001ea1;

/// U+1EA2 LATIN CAPITAL LETTER A WITH HOOK ABOVE
pub const FCITX_KEY_Ahook: u32 = 0x1001ea2;

/// U+1EA3 LATIN SMALL LETTER A WITH HOOK ABOVE
pub const FCITX_KEY_ahook: u32 = 0x1001ea3;

/// U+1EA4 LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND ACUTE
pub const FCITX_KEY_Acircumflexacute: u32 = 0x1001ea4;

/// U+1EA5 LATIN SMALL LETTER A WITH CIRCUMFLEX AND ACUTE
pub const FCITX_KEY_acircumflexacute: u32 = 0x1001ea5;

/// U+1EA6 LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND GRAVE
pub const FCITX_KEY_Acircumflexgrave: u32 = 0x1001ea6;

/// U+1EA7 LATIN SMALL LETTER A WITH CIRCUMFLEX AND GRAVE
pub const FCITX_KEY_acircumflexgrave: u32 = 0x1001ea7;

/// U+1EA8 LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND HOOK ABOVE
pub const FCITX_KEY_Acircumflexhook: u32 = 0x1001ea8;

/// U+1EA9 LATIN SMALL LETTER A WITH CIRCUMFLEX AND HOOK ABOVE
pub const FCITX_KEY_acircumflexhook: u32 = 0x1001ea9;

/// U+1EAA LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND TILDE
pub const FCITX_KEY_Acircumflextilde: u32 = 0x1001eaa;

/// U+1EAB LATIN SMALL LETTER A WITH CIRCUMFLEX AND TILDE
pub const FCITX_KEY_acircumflextilde: u32 = 0x1001eab;

/// U+1EAC LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND DOT BELOW
pub const FCITX_KEY_Acircumflexbelowdot: u32 = 0x1001eac;

/// U+1EAD LATIN SMALL LETTER A WITH CIRCUMFLEX AND DOT BELOW
pub const FCITX_KEY_acircumflexbelowdot: u32 = 0x1001ead;

/// U+1EAE LATIN CAPITAL LETTER A WITH BREVE AND ACUTE
pub const FCITX_KEY_Abreveacute: u32 = 0x1001eae;

/// U+1EAF LATIN SMALL LETTER A WITH BREVE AND ACUTE
pub const FCITX_KEY_abreveacute: u32 = 0x1001eaf;

/// U+1EB0 LATIN CAPITAL LETTER A WITH BREVE AND GRAVE
pub const FCITX_KEY_Abrevegrave: u32 = 0x1001eb0;

/// U+1EB1 LATIN SMALL LETTER A WITH BREVE AND GRAVE
pub const FCITX_KEY_abrevegrave: u32 = 0x1001eb1;

/// U+1EB2 LATIN CAPITAL LETTER A WITH BREVE AND HOOK ABOVE
pub const FCITX_KEY_Abrevehook: u32 = 0x1001eb2;

/// U+1EB3 LATIN SMALL LETTER A WITH BREVE AND HOOK ABOVE
pub const FCITX_KEY_abrevehook: u32 = 0x1001eb3;

/// U+1EB4 LATIN CAPITAL LETTER A WITH BREVE AND TILDE
pub const FCITX_KEY_Abrevetilde: u32 = 0x1001eb4;

/// U+1EB5 LATIN SMALL LETTER A WITH BREVE AND TILDE
pub const FCITX_KEY_abrevetilde: u32 = 0x1001eb5;

/// U+1EB6 LATIN CAPITAL LETTER A WITH BREVE AND DOT BELOW
pub const FCITX_KEY_Abrevebelowdot: u32 = 0x1001eb6;

/// U+1EB7 LATIN SMALL LETTER A WITH BREVE AND DOT BELOW
pub const FCITX_KEY_abrevebelowdot: u32 = 0x1001eb7;

/// U+1EB8 LATIN CAPITAL LETTER E WITH DOT BELOW
pub const FCITX_KEY_Ebelowdot: u32 = 0x1001eb8;

/// U+1EB9 LATIN SMALL LETTER E WITH DOT BELOW
pub const FCITX_KEY_ebelowdot: u32 = 0x1001eb9;

/// U+1EBA LATIN CAPITAL LETTER E WITH HOOK ABOVE
pub const FCITX_KEY_Ehook: u32 = 0x1001eba;

/// U+1EBB LATIN SMALL LETTER E WITH HOOK ABOVE
pub const FCITX_KEY_ehook: u32 = 0x1001ebb;

/// U+1EBC LATIN CAPITAL LETTER E WITH TILDE
pub const FCITX_KEY_Etilde: u32 = 0x1001ebc;

/// U+1EBD LATIN SMALL LETTER E WITH TILDE
pub const FCITX_KEY_etilde: u32 = 0x1001ebd;

/// U+1EBE LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND ACUTE
pub const FCITX_KEY_Ecircumflexacute: u32 = 0x1001ebe;

/// U+1EBF LATIN SMALL LETTER E WITH CIRCUMFLEX AND ACUTE
pub const FCITX_KEY_ecircumflexacute: u32 = 0x1001ebf;

/// U+1EC0 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND GRAVE
pub const FCITX_KEY_Ecircumflexgrave: u32 = 0x1001ec0;

/// U+1EC1 LATIN SMALL LETTER E WITH CIRCUMFLEX AND GRAVE
pub const FCITX_KEY_ecircumflexgrave: u32 = 0x1001ec1;

/// U+1EC2 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND HOOK ABOVE
pub const FCITX_KEY_Ecircumflexhook: u32 = 0x1001ec2;

/// U+1EC3 LATIN SMALL LETTER E WITH CIRCUMFLEX AND HOOK ABOVE
pub const FCITX_KEY_ecircumflexhook: u32 = 0x1001ec3;

/// U+1EC4 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND TILDE
pub const FCITX_KEY_Ecircumflextilde: u32 = 0x1001ec4;

/// U+1EC5 LATIN SMALL LETTER E WITH CIRCUMFLEX AND TILDE
pub const FCITX_KEY_ecircumflextilde: u32 = 0x1001ec5;

/// U+1EC6 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND DOT BELOW
pub const FCITX_KEY_Ecircumflexbelowdot: u32 = 0x1001ec6;

/// U+1EC7 LATIN SMALL LETTER E WITH CIRCUMFLEX AND DOT BELOW
pub const FCITX_KEY_ecircumflexbelowdot: u32 = 0x1001ec7;

/// U+1EC8 LATIN CAPITAL LETTER I WITH HOOK ABOVE
pub const FCITX_KEY_Ihook: u32 = 0x1001ec8;

/// U+1EC9 LATIN SMALL LETTER I WITH HOOK ABOVE
pub const FCITX_KEY_ihook: u32 = 0x1001ec9;

/// U+1ECA LATIN CAPITAL LETTER I WITH DOT BELOW
pub const FCITX_KEY_Ibelowdot: u32 = 0x1001eca;

/// U+1ECB LATIN SMALL LETTER I WITH DOT BELOW
pub const FCITX_KEY_ibelowdot: u32 = 0x1001ecb;

/// U+1ECC LATIN CAPITAL LETTER O WITH DOT BELOW
pub const FCITX_KEY_Obelowdot: u32 = 0x1001ecc;

/// U+1ECD LATIN SMALL LETTER O WITH DOT BELOW
pub const FCITX_KEY_obelowdot: u32 = 0x1001ecd;

/// U+1ECE LATIN CAPITAL LETTER O WITH HOOK ABOVE
pub const FCITX_KEY_Ohook: u32 = 0x1001ece;

/// U+1ECF LATIN SMALL LETTER O WITH HOOK ABOVE
pub const FCITX_KEY_ohook: u32 = 0x1001ecf;

/// U+1ED0 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND ACUTE
pub const FCITX_KEY_Ocircumflexacute: u32 = 0x1001ed0;

/// U+1ED1 LATIN SMALL LETTER O WITH CIRCUMFLEX AND ACUTE
pub const FCITX_KEY_ocircumflexacute: u32 = 0x1001ed1;

/// U+1ED2 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND GRAVE
pub const FCITX_KEY_Ocircumflexgrave: u32 = 0x1001ed2;

/// U+1ED3 LATIN SMALL LETTER O WITH CIRCUMFLEX AND GRAVE
pub const FCITX_KEY_ocircumflexgrave: u32 = 0x1001ed3;

/// U+1ED4 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND HOOK ABOVE
pub const FCITX_KEY_Ocircumflexhook: u32 = 0x1001ed4;

/// U+1ED5 LATIN SMALL LETTER O WITH CIRCUMFLEX AND HOOK ABOVE
pub const FCITX_KEY_ocircumflexhook: u32 = 0x1001ed5;

/// U+1ED6 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND TILDE
pub const FCITX_KEY_Ocircumflextilde: u32 = 0x1001ed6;

/// U+1ED7 LATIN SMALL LETTER O WITH CIRCUMFLEX AND TILDE
pub const FCITX_KEY_ocircumflextilde: u32 = 0x1001ed7;

/// U+1ED8 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND DOT BELOW
pub const FCITX_KEY_Ocircumflexbelowdot: u32 = 0x1001ed8;

/// U+1ED9 LATIN SMALL LETTER O WITH CIRCUMFLEX AND DOT BELOW
pub const FCITX_KEY_ocircumflexbelowdot: u32 = 0x1001ed9;

/// U+1EDA LATIN CAPITAL LETTER O WITH HORN AND ACUTE
pub const FCITX_KEY_Ohornacute: u32 = 0x1001eda;

/// U+1EDB LATIN SMALL LETTER O WITH HORN AND ACUTE
pub const FCITX_KEY_ohornacute: u32 = 0x1001edb;

/// U+1EDC LATIN CAPITAL LETTER O WITH HORN AND GRAVE
pub const FCITX_KEY_Ohorngrave: u32 = 0x1001edc;

/// U+1EDD LATIN SMALL LETTER O WITH HORN AND GRAVE
pub const FCITX_KEY_ohorngrave: u32 = 0x1001edd;

/// U+1EDE LATIN CAPITAL LETTER O WITH HORN AND HOOK ABOVE
pub const FCITX_KEY_Ohornhook: u32 = 0x1001ede;

/// U+1EDF LATIN SMALL LETTER O WITH HORN AND HOOK ABOVE
pub const FCITX_KEY_ohornhook: u32 = 0x1001edf;

/// U+1EE0 LATIN CAPITAL LETTER O WITH HORN AND TILDE
pub const FCITX_KEY_Ohorntilde: u32 = 0x1001ee0;

/// U+1EE1 LATIN SMALL LETTER O WITH HORN AND TILDE
pub const FCITX_KEY_ohorntilde: u32 = 0x1001ee1;

/// U+1EE2 LATIN CAPITAL LETTER O WITH HORN AND DOT BELOW
pub const FCITX_KEY_Ohornbelowdot: u32 = 0x1001ee2;

/// U+1EE3 LATIN SMALL LETTER O WITH HORN AND DOT BELOW
pub const FCITX_KEY_ohornbelowdot: u32 = 0x1001ee3;

/// U+1EE4 LATIN CAPITAL LETTER U WITH DOT BELOW
pub const FCITX_KEY_Ubelowdot: u32 = 0x1001ee4;

/// U+1EE5 LATIN SMALL LETTER U WITH DOT BELOW
pub const FCITX_KEY_ubelowdot: u32 = 0x1001ee5;

/// U+1EE6 LATIN CAPITAL LETTER U WITH HOOK ABOVE
pub const FCITX_KEY_Uhook: u32 = 0x1001ee6;

/// U+1EE7 LATIN SMALL LETTER U WITH HOOK ABOVE
pub const FCITX_KEY_uhook: u32 = 0x1001ee7;

/// U+1EE8 LATIN CAPITAL LETTER U WITH HORN AND ACUTE
pub const FCITX_KEY_Uhornacute: u32 = 0x1001ee8;

/// U+1EE9 LATIN SMALL LETTER U WITH HORN AND ACUTE
pub const FCITX_KEY_uhornacute: u32 = 0x1001ee9;

/// U+1EEA LATIN CAPITAL LETTER U WITH HORN AND GRAVE
pub const FCITX_KEY_Uhorngrave: u32 = 0x1001eea;

/// U+1EEB LATIN SMALL LETTER U WITH HORN AND GRAVE
pub const FCITX_KEY_uhorngrave: u32 = 0x1001eeb;

/// U+1EEC LATIN CAPITAL LETTER U WITH HORN AND HOOK ABOVE
pub const FCITX_KEY_Uhornhook: u32 = 0x1001eec;

/// U+1EED LATIN SMALL LETTER U WITH HORN AND HOOK ABOVE
pub const FCITX_KEY_uhornhook: u32 = 0x1001eed;

/// U+1EEE LATIN CAPITAL LETTER U WITH HORN AND TILDE
pub const FCITX_KEY_Uhorntilde: u32 = 0x1001eee;

/// U+1EEF LATIN SMALL LETTER U WITH HORN AND TILDE
pub const FCITX_KEY_uhorntilde: u32 = 0x1001eef;

/// U+1EF0 LATIN CAPITAL LETTER U WITH HORN AND DOT BELOW
pub const FCITX_KEY_Uhornbelowdot: u32 = 0x1001ef0;

/// U+1EF1 LATIN SMALL LETTER U WITH HORN AND DOT BELOW
pub const FCITX_KEY_uhornbelowdot: u32 = 0x1001ef1;

/// U+1EF4 LATIN CAPITAL LETTER Y WITH DOT BELOW
pub const FCITX_KEY_Ybelowdot: u32 = 0x1001ef4;

/// U+1EF5 LATIN SMALL LETTER Y WITH DOT BELOW
pub const FCITX_KEY_ybelowdot: u32 = 0x1001ef5;

/// U+1EF6 LATIN CAPITAL LETTER Y WITH HOOK ABOVE
pub const FCITX_KEY_Yhook: u32 = 0x1001ef6;

/// U+1EF7 LATIN SMALL LETTER Y WITH HOOK ABOVE
pub const FCITX_KEY_yhook: u32 = 0x1001ef7;

/// U+1EF8 LATIN CAPITAL LETTER Y WITH TILDE
pub const FCITX_KEY_Ytilde: u32 = 0x1001ef8;

/// U+1EF9 LATIN SMALL LETTER Y WITH TILDE
pub const FCITX_KEY_ytilde: u32 = 0x1001ef9;

/// U+01A0 LATIN CAPITAL LETTER O WITH HORN
pub const FCITX_KEY_Ohorn: u32 = 0x10001a0;

/// U+01A1 LATIN SMALL LETTER O WITH HORN
pub const FCITX_KEY_ohorn: u32 = 0x10001a1;

/// U+01AF LATIN CAPITAL LETTER U WITH HORN
pub const FCITX_KEY_Uhorn: u32 = 0x10001af;

/// U+01B0 LATIN SMALL LETTER U WITH HORN
pub const FCITX_KEY_uhorn: u32 = 0x10001b0;

/// U+0303 COMBINING TILDE
pub const FCITX_KEY_combining_tilde: u32 = 0x1000303;

/// U+0300 COMBINING GRAVE ACCENT
pub const FCITX_KEY_combining_grave: u32 = 0x1000300;

/// U+0301 COMBINING ACUTE ACCENT
pub const FCITX_KEY_combining_acute: u32 = 0x1000301;

/// U+0309 COMBINING HOOK ABOVE
pub const FCITX_KEY_combining_hook: u32 = 0x1000309;

/// U+0323 COMBINING DOT BELOW
pub const FCITX_KEY_combining_belowdot: u32 = 0x1000323;

/// U+20A0 EURO-CURRENCY SIGN
pub const FCITX_KEY_EcuSign: u32 = 0x10020a0;

/// U+20A1 COLON SIGN
pub const FCITX_KEY_ColonSign: u32 = 0x10020a1;

/// U+20A2 CRUZEIRO SIGN
pub const FCITX_KEY_CruzeiroSign: u32 = 0x10020a2;

/// U+20A3 FRENCH FRANC SIGN
pub const FCITX_KEY_FFrancSign: u32 = 0x10020a3;

/// U+20A4 LIRA SIGN
pub const FCITX_KEY_LiraSign: u32 = 0x10020a4;

/// U+20A5 MILL SIGN
pub const FCITX_KEY_MillSign: u32 = 0x10020a5;

/// U+20A6 NAIRA SIGN
pub const FCITX_KEY_NairaSign: u32 = 0x10020a6;

/// U+20A7 PESETA SIGN
pub const FCITX_KEY_PesetaSign: u32 = 0x10020a7;

/// U+20A8 RUPEE SIGN
pub const FCITX_KEY_RupeeSign: u32 = 0x10020a8;

/// U+20A9 WON SIGN
pub const FCITX_KEY_WonSign: u32 = 0x10020a9;

/// U+20AA NEW SHEQEL SIGN
pub const FCITX_KEY_NewSheqelSign: u32 = 0x10020aa;

/// U+20AB DONG SIGN
pub const FCITX_KEY_DongSign: u32 = 0x10020ab;

/// U+20AC EURO SIGN
pub const FCITX_KEY_EuroSign: u32 = 0x20ac;

/// U+2070 SUPERSCRIPT ZERO
pub const FCITX_KEY_zerosuperior: u32 = 0x1002070;

/// U+2074 SUPERSCRIPT FOUR
pub const FCITX_KEY_foursuperior: u32 = 0x1002074;

/// U+2075 SUPERSCRIPT FIVE
pub const FCITX_KEY_fivesuperior: u32 = 0x1002075;

/// U+2076 SUPERSCRIPT SIX
pub const FCITX_KEY_sixsuperior: u32 = 0x1002076;

/// U+2077 SUPERSCRIPT SEVEN
pub const FCITX_KEY_sevensuperior: u32 = 0x1002077;

/// U+2078 SUPERSCRIPT EIGHT
pub const FCITX_KEY_eightsuperior: u32 = 0x1002078;

/// U+2079 SUPERSCRIPT NINE
pub const FCITX_KEY_ninesuperior: u32 = 0x1002079;

/// U+2080 SUBSCRIPT ZERO
pub const FCITX_KEY_zerosubscript: u32 = 0x1002080;

/// U+2081 SUBSCRIPT ONE
pub const FCITX_KEY_onesubscript: u32 = 0x1002081;

/// U+2082 SUBSCRIPT TWO
pub const FCITX_KEY_twosubscript: u32 = 0x1002082;

/// U+2083 SUBSCRIPT THREE
pub const FCITX_KEY_threesubscript: u32 = 0x1002083;

/// U+2084 SUBSCRIPT FOUR
pub const FCITX_KEY_foursubscript: u32 = 0x1002084;

/// U+2085 SUBSCRIPT FIVE
pub const FCITX_KEY_fivesubscript: u32 = 0x1002085;

/// U+2086 SUBSCRIPT SIX
pub const FCITX_KEY_sixsubscript: u32 = 0x1002086;

/// U+2087 SUBSCRIPT SEVEN
pub const FCITX_KEY_sevensubscript: u32 = 0x1002087;

/// U+2088 SUBSCRIPT EIGHT
pub const FCITX_KEY_eightsubscript: u32 = 0x1002088;

/// U+2089 SUBSCRIPT NINE
pub const FCITX_KEY_ninesubscript: u32 = 0x1002089;

/// U+2202 PARTIAL DIFFERENTIAL
pub const FCITX_KEY_partdifferential: u32 = 0x1002202;

/// U+2205 EMPTY SET
pub const FCITX_KEY_emptyset: u32 = 0x1002205;

/// U+2208 ELEMENT OF
pub const FCITX_KEY_elementof: u32 = 0x1002208;

/// U+2209 NOT AN ELEMENT OF
pub const FCITX_KEY_notelementof: u32 = 0x1002209;

/// U+220B CONTAINS AS MEMBER
pub const FCITX_KEY_containsas: u32 = 0x100220b;

/// U+221A SQUARE ROOT
pub const FCITX_KEY_squareroot: u32 = 0x100221a;

/// U+221B CUBE ROOT
pub const FCITX_KEY_cuberoot: u32 = 0x100221b;

/// U+221C FOURTH ROOT
pub const FCITX_KEY_fourthroot: u32 = 0x100221c;

/// U+222C DOUBLE INTEGRAL
pub const FCITX_KEY_dintegral: u32 = 0x100222c;

/// U+222D TRIPLE INTEGRAL
pub const FCITX_KEY_tintegral: u32 = 0x100222d;

/// U+2235 BECAUSE
pub const FCITX_KEY_because: u32 = 0x1002235;

/// (U+2248 ALMOST EQUAL TO)
pub const FCITX_KEY_approxeq: u32 = 0x1002248;

/// (U+2247 NEITHER APPROXIMATELY NOR ACTUALLY EQUAL TO)
pub const FCITX_KEY_notapproxeq: u32 = 0x1002247;

/// U+2262 NOT IDENTICAL TO
pub const FCITX_KEY_notidentical: u32 = 0x1002262;

/// U+2263 STRICTLY EQUIVALENT TO
pub const FCITX_KEY_stricteq: u32 = 0x1002263;

pub const FCITX_KEY_braille_dot_1: u32 = 0xfff1;

pub const FCITX_KEY_braille_dot_2: u32 = 0xfff2;

pub const FCITX_KEY_braille_dot_3: u32 = 0xfff3;

pub const FCITX_KEY_braille_dot_4: u32 = 0xfff4;

pub const FCITX_KEY_braille_dot_5: u32 = 0xfff5;

pub const FCITX_KEY_braille_dot_6: u32 = 0xfff6;

pub const FCITX_KEY_braille_dot_7: u32 = 0xfff7;

pub const FCITX_KEY_braille_dot_8: u32 = 0xfff8;

pub const FCITX_KEY_braille_dot_9: u32 = 0xfff9;

pub const FCITX_KEY_braille_dot_10: u32 = 0xfffa;

/// U+2800 BRAILLE PATTERN BLANK
pub const FCITX_KEY_braille_blank: u32 = 0x1002800;

/// U+2801 BRAILLE PATTERN DOTS-1
pub const FCITX_KEY_braille_dots_1: u32 = 0x1002801;

/// U+2802 BRAILLE PATTERN DOTS-2
pub const FCITX_KEY_braille_dots_2: u32 = 0x1002802;

/// U+2803 BRAILLE PATTERN DOTS-12
pub const FCITX_KEY_braille_dots_12: u32 = 0x1002803;

/// U+2804 BRAILLE PATTERN DOTS-3
pub const FCITX_KEY_braille_dots_3: u32 = 0x1002804;

/// U+2805 BRAILLE PATTERN DOTS-13
pub const FCITX_KEY_braille_dots_13: u32 = 0x1002805;

/// U+2806 BRAILLE PATTERN DOTS-23
pub const FCITX_KEY_braille_dots_23: u32 = 0x1002806;

/// U+2807 BRAILLE PATTERN DOTS-123
pub const FCITX_KEY_braille_dots_123: u32 = 0x1002807;

/// U+2808 BRAILLE PATTERN DOTS-4
pub const FCITX_KEY_braille_dots_4: u32 = 0x1002808;

/// U+2809 BRAILLE PATTERN DOTS-14
pub const FCITX_KEY_braille_dots_14: u32 = 0x1002809;

/// U+280A BRAILLE PATTERN DOTS-24
pub const FCITX_KEY_braille_dots_24: u32 = 0x100280a;

/// U+280B BRAILLE PATTERN DOTS-124
pub const FCITX_KEY_braille_dots_124: u32 = 0x100280b;

/// U+280C BRAILLE PATTERN DOTS-34
pub const FCITX_KEY_braille_dots_34: u32 = 0x100280c;

/// U+280D BRAILLE PATTERN DOTS-134
pub const FCITX_KEY_braille_dots_134: u32 = 0x100280d;

/// U+280E BRAILLE PATTERN DOTS-234
pub const FCITX_KEY_braille_dots_234: u32 = 0x100280e;

/// U+280F BRAILLE PATTERN DOTS-1234
pub const FCITX_KEY_braille_dots_1234: u32 = 0x100280f;

/// U+2810 BRAILLE PATTERN DOTS-5
pub const FCITX_KEY_braille_dots_5: u32 = 0x1002810;

/// U+2811 BRAILLE PATTERN DOTS-15
pub const FCITX_KEY_braille_dots_15: u32 = 0x1002811;

/// U+2812 BRAILLE PATTERN DOTS-25
pub const FCITX_KEY_braille_dots_25: u32 = 0x1002812;

/// U+2813 BRAILLE PATTERN DOTS-125
pub const FCITX_KEY_braille_dots_125: u32 = 0x1002813;

/// U+2814 BRAILLE PATTERN DOTS-35
pub const FCITX_KEY_braille_dots_35: u32 = 0x1002814;

/// U+2815 BRAILLE PATTERN DOTS-135
pub const FCITX_KEY_braille_dots_135: u32 = 0x1002815;

/// U+2816 BRAILLE PATTERN DOTS-235
pub const FCITX_KEY_braille_dots_235: u32 = 0x1002816;

/// U+2817 BRAILLE PATTERN DOTS-1235
pub const FCITX_KEY_braille_dots_1235: u32 = 0x1002817;

/// U+2818 BRAILLE PATTERN DOTS-45
pub const FCITX_KEY_braille_dots_45: u32 = 0x1002818;

/// U+2819 BRAILLE PATTERN DOTS-145
pub const FCITX_KEY_braille_dots_145: u32 = 0x1002819;

/// U+281A BRAILLE PATTERN DOTS-245
pub const FCITX_KEY_braille_dots_245: u32 = 0x100281a;

/// U+281B BRAILLE PATTERN DOTS-1245
pub const FCITX_KEY_braille_dots_1245: u32 = 0x100281b;

/// U+281C BRAILLE PATTERN DOTS-345
pub const FCITX_KEY_braille_dots_345: u32 = 0x100281c;

/// U+281D BRAILLE PATTERN DOTS-1345
pub const FCITX_KEY_braille_dots_1345: u32 = 0x100281d;

/// U+281E BRAILLE PATTERN DOTS-2345
pub const FCITX_KEY_braille_dots_2345: u32 = 0x100281e;

/// U+281F BRAILLE PATTERN DOTS-12345
pub const FCITX_KEY_braille_dots_12345: u32 = 0x100281f;

/// U+2820 BRAILLE PATTERN DOTS-6
pub const FCITX_KEY_braille_dots_6: u32 = 0x1002820;

/// U+2821 BRAILLE PATTERN DOTS-16
pub const FCITX_KEY_braille_dots_16: u32 = 0x1002821;

/// U+2822 BRAILLE PATTERN DOTS-26
pub const FCITX_KEY_braille_dots_26: u32 = 0x1002822;

/// U+2823 BRAILLE PATTERN DOTS-126
pub const FCITX_KEY_braille_dots_126: u32 = 0x1002823;

/// U+2824 BRAILLE PATTERN DOTS-36
pub const FCITX_KEY_braille_dots_36: u32 = 0x1002824;

/// U+2825 BRAILLE PATTERN DOTS-136
pub const FCITX_KEY_braille_dots_136: u32 = 0x1002825;

/// U+2826 BRAILLE PATTERN DOTS-236
pub const FCITX_KEY_braille_dots_236: u32 = 0x1002826;

/// U+2827 BRAILLE PATTERN DOTS-1236
pub const FCITX_KEY_braille_dots_1236: u32 = 0x1002827;

/// U+2828 BRAILLE PATTERN DOTS-46
pub const FCITX_KEY_braille_dots_46: u32 = 0x1002828;

/// U+2829 BRAILLE PATTERN DOTS-146
pub const FCITX_KEY_braille_dots_146: u32 = 0x1002829;

/// U+282A BRAILLE PATTERN DOTS-246
pub const FCITX_KEY_braille_dots_246: u32 = 0x100282a;

/// U+282B BRAILLE PATTERN DOTS-1246
pub const FCITX_KEY_braille_dots_1246: u32 = 0x100282b;

/// U+282C BRAILLE PATTERN DOTS-346
pub const FCITX_KEY_braille_dots_346: u32 = 0x100282c;

/// U+282D BRAILLE PATTERN DOTS-1346
pub const FCITX_KEY_braille_dots_1346: u32 = 0x100282d;

/// U+282E BRAILLE PATTERN DOTS-2346
pub const FCITX_KEY_braille_dots_2346: u32 = 0x100282e;

/// U+282F BRAILLE PATTERN DOTS-12346
pub const FCITX_KEY_braille_dots_12346: u32 = 0x100282f;

/// U+2830 BRAILLE PATTERN DOTS-56
pub const FCITX_KEY_braille_dots_56: u32 = 0x1002830;

/// U+2831 BRAILLE PATTERN DOTS-156
pub const FCITX_KEY_braille_dots_156: u32 = 0x1002831;

/// U+2832 BRAILLE PATTERN DOTS-256
pub const FCITX_KEY_braille_dots_256: u32 = 0x1002832;

/// U+2833 BRAILLE PATTERN DOTS-1256
pub const FCITX_KEY_braille_dots_1256: u32 = 0x1002833;

/// U+2834 BRAILLE PATTERN DOTS-356
pub const FCITX_KEY_braille_dots_356: u32 = 0x1002834;

/// U+2835 BRAILLE PATTERN DOTS-1356
pub const FCITX_KEY_braille_dots_1356: u32 = 0x1002835;

/// U+2836 BRAILLE PATTERN DOTS-2356
pub const FCITX_KEY_braille_dots_2356: u32 = 0x1002836;

/// U+2837 BRAILLE PATTERN DOTS-12356
pub const FCITX_KEY_braille_dots_12356: u32 = 0x1002837;

/// U+2838 BRAILLE PATTERN DOTS-456
pub const FCITX_KEY_braille_dots_456: u32 = 0x1002838;

/// U+2839 BRAILLE PATTERN DOTS-1456
pub const FCITX_KEY_braille_dots_1456: u32 = 0x1002839;

/// U+283A BRAILLE PATTERN DOTS-2456
pub const FCITX_KEY_braille_dots_2456: u32 = 0x100283a;

/// U+283B BRAILLE PATTERN DOTS-12456
pub const FCITX_KEY_braille_dots_12456: u32 = 0x100283b;

/// U+283C BRAILLE PATTERN DOTS-3456
pub const FCITX_KEY_braille_dots_3456: u32 = 0x100283c;

/// U+283D BRAILLE PATTERN DOTS-13456
pub const FCITX_KEY_braille_dots_13456: u32 = 0x100283d;

/// U+283E BRAILLE PATTERN DOTS-23456
pub const FCITX_KEY_braille_dots_23456: u32 = 0x100283e;

/// U+283F BRAILLE PATTERN DOTS-123456
pub const FCITX_KEY_braille_dots_123456: u32 = 0x100283f;

/// U+2840 BRAILLE PATTERN DOTS-7
pub const FCITX_KEY_braille_dots_7: u32 = 0x1002840;

/// U+2841 BRAILLE PATTERN DOTS-17
pub const FCITX_KEY_braille_dots_17: u32 = 0x1002841;

/// U+2842 BRAILLE PATTERN DOTS-27
pub const FCITX_KEY_braille_dots_27: u32 = 0x1002842;

/// U+2843 BRAILLE PATTERN DOTS-127
pub const FCITX_KEY_braille_dots_127: u32 = 0x1002843;

/// U+2844 BRAILLE PATTERN DOTS-37
pub const FCITX_KEY_braille_dots_37: u32 = 0x1002844;

/// U+2845 BRAILLE PATTERN DOTS-137
pub const FCITX_KEY_braille_dots_137: u32 = 0x1002845;

/// U+2846 BRAILLE PATTERN DOTS-237
pub const FCITX_KEY_braille_dots_237: u32 = 0x1002846;

/// U+2847 BRAILLE PATTERN DOTS-1237
pub const FCITX_KEY_braille_dots_1237: u32 = 0x1002847;

/// U+2848 BRAILLE PATTERN DOTS-47
pub const FCITX_KEY_braille_dots_47: u32 = 0x1002848;

/// U+2849 BRAILLE PATTERN DOTS-147
pub const FCITX_KEY_braille_dots_147: u32 = 0x1002849;

/// U+284A BRAILLE PATTERN DOTS-247
pub const FCITX_KEY_braille_dots_247: u32 = 0x100284a;

/// U+284B BRAILLE PATTERN DOTS-1247
pub const FCITX_KEY_braille_dots_1247: u32 = 0x100284b;

/// U+284C BRAILLE PATTERN DOTS-347
pub const FCITX_KEY_braille_dots_347: u32 = 0x100284c;

/// U+284D BRAILLE PATTERN DOTS-1347
pub const FCITX_KEY_braille_dots_1347: u32 = 0x100284d;

/// U+284E BRAILLE PATTERN DOTS-2347
pub const FCITX_KEY_braille_dots_2347: u32 = 0x100284e;

/// U+284F BRAILLE PATTERN DOTS-12347
pub const FCITX_KEY_braille_dots_12347: u32 = 0x100284f;

/// U+2850 BRAILLE PATTERN DOTS-57
pub const FCITX_KEY_braille_dots_57: u32 = 0x1002850;

/// U+2851 BRAILLE PATTERN DOTS-157
pub const FCITX_KEY_braille_dots_157: u32 = 0x1002851;

/// U+2852 BRAILLE PATTERN DOTS-257
pub const FCITX_KEY_braille_dots_257: u32 = 0x1002852;

/// U+2853 BRAILLE PATTERN DOTS-1257
pub const FCITX_KEY_braille_dots_1257: u32 = 0x1002853;

/// U+2854 BRAILLE PATTERN DOTS-357
pub const FCITX_KEY_braille_dots_357: u32 = 0x1002854;

/// U+2855 BRAILLE PATTERN DOTS-1357
pub const FCITX_KEY_braille_dots_1357: u32 = 0x1002855;

/// U+2856 BRAILLE PATTERN DOTS-2357
pub const FCITX_KEY_braille_dots_2357: u32 = 0x1002856;

/// U+2857 BRAILLE PATTERN DOTS-12357
pub const FCITX_KEY_braille_dots_12357: u32 = 0x1002857;

/// U+2858 BRAILLE PATTERN DOTS-457
pub const FCITX_KEY_braille_dots_457: u32 = 0x1002858;

/// U+2859 BRAILLE PATTERN DOTS-1457
pub const FCITX_KEY_braille_dots_1457: u32 = 0x1002859;

/// U+285A BRAILLE PATTERN DOTS-2457
pub const FCITX_KEY_braille_dots_2457: u32 = 0x100285a;

/// U+285B BRAILLE PATTERN DOTS-12457
pub const FCITX_KEY_braille_dots_12457: u32 = 0x100285b;

/// U+285C BRAILLE PATTERN DOTS-3457
pub const FCITX_KEY_braille_dots_3457: u32 = 0x100285c;

/// U+285D BRAILLE PATTERN DOTS-13457
pub const FCITX_KEY_braille_dots_13457: u32 = 0x100285d;

/// U+285E BRAILLE PATTERN DOTS-23457
pub const FCITX_KEY_braille_dots_23457: u32 = 0x100285e;

/// U+285F BRAILLE PATTERN DOTS-123457
pub const FCITX_KEY_braille_dots_123457: u32 = 0x100285f;

/// U+2860 BRAILLE PATTERN DOTS-67
pub const FCITX_KEY_braille_dots_67: u32 = 0x1002860;

/// U+2861 BRAILLE PATTERN DOTS-167
pub const FCITX_KEY_braille_dots_167: u32 = 0x1002861;

/// U+2862 BRAILLE PATTERN DOTS-267
pub const FCITX_KEY_braille_dots_267: u32 = 0x1002862;

/// U+2863 BRAILLE PATTERN DOTS-1267
pub const FCITX_KEY_braille_dots_1267: u32 = 0x1002863;

/// U+2864 BRAILLE PATTERN DOTS-367
pub const FCITX_KEY_braille_dots_367: u32 = 0x1002864;

/// U+2865 BRAILLE PATTERN DOTS-1367
pub const FCITX_KEY_braille_dots_1367: u32 = 0x1002865;

/// U+2866 BRAILLE PATTERN DOTS-2367
pub const FCITX_KEY_braille_dots_2367: u32 = 0x1002866;

/// U+2867 BRAILLE PATTERN DOTS-12367
pub const FCITX_KEY_braille_dots_12367: u32 = 0x1002867;

/// U+2868 BRAILLE PATTERN DOTS-467
pub const FCITX_KEY_braille_dots_467: u32 = 0x1002868;

/// U+2869 BRAILLE PATTERN DOTS-1467
pub const FCITX_KEY_braille_dots_1467: u32 = 0x1002869;

/// U+286A BRAILLE PATTERN DOTS-2467
pub const FCITX_KEY_braille_dots_2467: u32 = 0x100286a;

/// U+286B BRAILLE PATTERN DOTS-12467
pub const FCITX_KEY_braille_dots_12467: u32 = 0x100286b;

/// U+286C BRAILLE PATTERN DOTS-3467
pub const FCITX_KEY_braille_dots_3467: u32 = 0x100286c;

/// U+286D BRAILLE PATTERN DOTS-13467
pub const FCITX_KEY_braille_dots_13467: u32 = 0x100286d;

/// U+286E BRAILLE PATTERN DOTS-23467
pub const FCITX_KEY_braille_dots_23467: u32 = 0x100286e;

/// U+286F BRAILLE PATTERN DOTS-123467
pub const FCITX_KEY_braille_dots_123467: u32 = 0x100286f;

/// U+2870 BRAILLE PATTERN DOTS-567
pub const FCITX_KEY_braille_dots_567: u32 = 0x1002870;

/// U+2871 BRAILLE PATTERN DOTS-1567
pub const FCITX_KEY_braille_dots_1567: u32 = 0x1002871;

/// U+2872 BRAILLE PATTERN DOTS-2567
pub const FCITX_KEY_braille_dots_2567: u32 = 0x1002872;

/// U+2873 BRAILLE PATTERN DOTS-12567
pub const FCITX_KEY_braille_dots_12567: u32 = 0x1002873;

/// U+2874 BRAILLE PATTERN DOTS-3567
pub const FCITX_KEY_braille_dots_3567: u32 = 0x1002874;

/// U+2875 BRAILLE PATTERN DOTS-13567
pub const FCITX_KEY_braille_dots_13567: u32 = 0x1002875;

/// U+2876 BRAILLE PATTERN DOTS-23567
pub const FCITX_KEY_braille_dots_23567: u32 = 0x1002876;

/// U+2877 BRAILLE PATTERN DOTS-123567
pub const FCITX_KEY_braille_dots_123567: u32 = 0x1002877;

/// U+2878 BRAILLE PATTERN DOTS-4567
pub const FCITX_KEY_braille_dots_4567: u32 = 0x1002878;

/// U+2879 BRAILLE PATTERN DOTS-14567
pub const FCITX_KEY_braille_dots_14567: u32 = 0x1002879;

/// U+287A BRAILLE PATTERN DOTS-24567
pub const FCITX_KEY_braille_dots_24567: u32 = 0x100287a;

/// U+287B BRAILLE PATTERN DOTS-124567
pub const FCITX_KEY_braille_dots_124567: u32 = 0x100287b;

/// U+287C BRAILLE PATTERN DOTS-34567
pub const FCITX_KEY_braille_dots_34567: u32 = 0x100287c;

/// U+287D BRAILLE PATTERN DOTS-134567
pub const FCITX_KEY_braille_dots_134567: u32 = 0x100287d;

/// U+287E BRAILLE PATTERN DOTS-234567
pub const FCITX_KEY_braille_dots_234567: u32 = 0x100287e;

/// U+287F BRAILLE PATTERN DOTS-1234567
pub const FCITX_KEY_braille_dots_1234567: u32 = 0x100287f;

/// U+2880 BRAILLE PATTERN DOTS-8
pub const FCITX_KEY_braille_dots_8: u32 = 0x1002880;

/// U+2881 BRAILLE PATTERN DOTS-18
pub const FCITX_KEY_braille_dots_18: u32 = 0x1002881;

/// U+2882 BRAILLE PATTERN DOTS-28
pub const FCITX_KEY_braille_dots_28: u32 = 0x1002882;

/// U+2883 BRAILLE PATTERN DOTS-128
pub const FCITX_KEY_braille_dots_128: u32 = 0x1002883;

/// U+2884 BRAILLE PATTERN DOTS-38
pub const FCITX_KEY_braille_dots_38: u32 = 0x1002884;

/// U+2885 BRAILLE PATTERN DOTS-138
pub const FCITX_KEY_braille_dots_138: u32 = 0x1002885;

/// U+2886 BRAILLE PATTERN DOTS-238
pub const FCITX_KEY_braille_dots_238: u32 = 0x1002886;

/// U+2887 BRAILLE PATTERN DOTS-1238
pub const FCITX_KEY_braille_dots_1238: u32 = 0x1002887;

/// U+2888 BRAILLE PATTERN DOTS-48
pub const FCITX_KEY_braille_dots_48: u32 = 0x1002888;

/// U+2889 BRAILLE PATTERN DOTS-148
pub const FCITX_KEY_braille_dots_148: u32 = 0x1002889;

/// U+288A BRAILLE PATTERN DOTS-248
pub const FCITX_KEY_braille_dots_248: u32 = 0x100288a;

/// U+288B BRAILLE PATTERN DOTS-1248
pub const FCITX_KEY_braille_dots_1248: u32 = 0x100288b;

/// U+288C BRAILLE PATTERN DOTS-348
pub const FCITX_KEY_braille_dots_348: u32 = 0x100288c;

/// U+288D BRAILLE PATTERN DOTS-1348
pub const FCITX_KEY_braille_dots_1348: u32 = 0x100288d;

/// U+288E BRAILLE PATTERN DOTS-2348
pub const FCITX_KEY_braille_dots_2348: u32 = 0x100288e;

/// U+288F BRAILLE PATTERN DOTS-12348
pub const FCITX_KEY_braille_dots_12348: u32 = 0x100288f;

/// U+2890 BRAILLE PATTERN DOTS-58
pub const FCITX_KEY_braille_dots_58: u32 = 0x1002890;

/// U+2891 BRAILLE PATTERN DOTS-158
pub const FCITX_KEY_braille_dots_158: u32 = 0x1002891;

/// U+2892 BRAILLE PATTERN DOTS-258
pub const FCITX_KEY_braille_dots_258: u32 = 0x1002892;

/// U+2893 BRAILLE PATTERN DOTS-1258
pub const FCITX_KEY_braille_dots_1258: u32 = 0x1002893;

/// U+2894 BRAILLE PATTERN DOTS-358
pub const FCITX_KEY_braille_dots_358: u32 = 0x1002894;

/// U+2895 BRAILLE PATTERN DOTS-1358
pub const FCITX_KEY_braille_dots_1358: u32 = 0x1002895;

/// U+2896 BRAILLE PATTERN DOTS-2358
pub const FCITX_KEY_braille_dots_2358: u32 = 0x1002896;

/// U+2897 BRAILLE PATTERN DOTS-12358
pub const FCITX_KEY_braille_dots_12358: u32 = 0x1002897;

/// U+2898 BRAILLE PATTERN DOTS-458
pub const FCITX_KEY_braille_dots_458: u32 = 0x1002898;

/// U+2899 BRAILLE PATTERN DOTS-1458
pub const FCITX_KEY_braille_dots_1458: u32 = 0x1002899;

/// U+289A BRAILLE PATTERN DOTS-2458
pub const FCITX_KEY_braille_dots_2458: u32 = 0x100289a;

/// U+289B BRAILLE PATTERN DOTS-12458
pub const FCITX_KEY_braille_dots_12458: u32 = 0x100289b;

/// U+289C BRAILLE PATTERN DOTS-3458
pub const FCITX_KEY_braille_dots_3458: u32 = 0x100289c;

/// U+289D BRAILLE PATTERN DOTS-13458
pub const FCITX_KEY_braille_dots_13458: u32 = 0x100289d;

/// U+289E BRAILLE PATTERN DOTS-23458
pub const FCITX_KEY_braille_dots_23458: u32 = 0x100289e;

/// U+289F BRAILLE PATTERN DOTS-123458
pub const FCITX_KEY_braille_dots_123458: u32 = 0x100289f;

/// U+28A0 BRAILLE PATTERN DOTS-68
pub const FCITX_KEY_braille_dots_68: u32 = 0x10028a0;

/// U+28A1 BRAILLE PATTERN DOTS-168
pub const FCITX_KEY_braille_dots_168: u32 = 0x10028a1;

/// U+28A2 BRAILLE PATTERN DOTS-268
pub const FCITX_KEY_braille_dots_268: u32 = 0x10028a2;

/// U+28A3 BRAILLE PATTERN DOTS-1268
pub const FCITX_KEY_braille_dots_1268: u32 = 0x10028a3;

/// U+28A4 BRAILLE PATTERN DOTS-368
pub const FCITX_KEY_braille_dots_368: u32 = 0x10028a4;

/// U+28A5 BRAILLE PATTERN DOTS-1368
pub const FCITX_KEY_braille_dots_1368: u32 = 0x10028a5;

/// U+28A6 BRAILLE PATTERN DOTS-2368
pub const FCITX_KEY_braille_dots_2368: u32 = 0x10028a6;

/// U+28A7 BRAILLE PATTERN DOTS-12368
pub const FCITX_KEY_braille_dots_12368: u32 = 0x10028a7;

/// U+28A8 BRAILLE PATTERN DOTS-468
pub const FCITX_KEY_braille_dots_468: u32 = 0x10028a8;

/// U+28A9 BRAILLE PATTERN DOTS-1468
pub const FCITX_KEY_braille_dots_1468: u32 = 0x10028a9;

/// U+28AA BRAILLE PATTERN DOTS-2468
pub const FCITX_KEY_braille_dots_2468: u32 = 0x10028aa;

/// U+28AB BRAILLE PATTERN DOTS-12468
pub const FCITX_KEY_braille_dots_12468: u32 = 0x10028ab;

/// U+28AC BRAILLE PATTERN DOTS-3468
pub const FCITX_KEY_braille_dots_3468: u32 = 0x10028ac;

/// U+28AD BRAILLE PATTERN DOTS-13468
pub const FCITX_KEY_braille_dots_13468: u32 = 0x10028ad;

/// U+28AE BRAILLE PATTERN DOTS-23468
pub const FCITX_KEY_braille_dots_23468: u32 = 0x10028ae;

/// U+28AF BRAILLE PATTERN DOTS-123468
pub const FCITX_KEY_braille_dots_123468: u32 = 0x10028af;

/// U+28B0 BRAILLE PATTERN DOTS-568
pub const FCITX_KEY_braille_dots_568: u32 = 0x10028b0;

/// U+28B1 BRAILLE PATTERN DOTS-1568
pub const FCITX_KEY_braille_dots_1568: u32 = 0x10028b1;

/// U+28B2 BRAILLE PATTERN DOTS-2568
pub const FCITX_KEY_braille_dots_2568: u32 = 0x10028b2;

/// U+28B3 BRAILLE PATTERN DOTS-12568
pub const FCITX_KEY_braille_dots_12568: u32 = 0x10028b3;

/// U+28B4 BRAILLE PATTERN DOTS-3568
pub const FCITX_KEY_braille_dots_3568: u32 = 0x10028b4;

/// U+28B5 BRAILLE PATTERN DOTS-13568
pub const FCITX_KEY_braille_dots_13568: u32 = 0x10028b5;

/// U+28B6 BRAILLE PATTERN DOTS-23568
pub const FCITX_KEY_braille_dots_23568: u32 = 0x10028b6;

/// U+28B7 BRAILLE PATTERN DOTS-123568
pub const FCITX_KEY_braille_dots_123568: u32 = 0x10028b7;

/// U+28B8 BRAILLE PATTERN DOTS-4568
pub const FCITX_KEY_braille_dots_4568: u32 = 0x10028b8;

/// U+28B9 BRAILLE PATTERN DOTS-14568
pub const FCITX_KEY_braille_dots_14568: u32 = 0x10028b9;

/// U+28BA BRAILLE PATTERN DOTS-24568
pub const FCITX_KEY_braille_dots_24568: u32 = 0x10028ba;

/// U+28BB BRAILLE PATTERN DOTS-124568
pub const FCITX_KEY_braille_dots_124568: u32 = 0x10028bb;

/// U+28BC BRAILLE PATTERN DOTS-34568
pub const FCITX_KEY_braille_dots_34568: u32 = 0x10028bc;

/// U+28BD BRAILLE PATTERN DOTS-134568
pub const FCITX_KEY_braille_dots_134568: u32 = 0x10028bd;

/// U+28BE BRAILLE PATTERN DOTS-234568
pub const FCITX_KEY_braille_dots_234568: u32 = 0x10028be;

/// U+28BF BRAILLE PATTERN DOTS-1234568
pub const FCITX_KEY_braille_dots_1234568: u32 = 0x10028bf;

/// U+28C0 BRAILLE PATTERN DOTS-78
pub const FCITX_KEY_braille_dots_78: u32 = 0x10028c0;

/// U+28C1 BRAILLE PATTERN DOTS-178
pub const FCITX_KEY_braille_dots_178: u32 = 0x10028c1;

/// U+28C2 BRAILLE PATTERN DOTS-278
pub const FCITX_KEY_braille_dots_278: u32 = 0x10028c2;

/// U+28C3 BRAILLE PATTERN DOTS-1278
pub const FCITX_KEY_braille_dots_1278: u32 = 0x10028c3;

/// U+28C4 BRAILLE PATTERN DOTS-378
pub const FCITX_KEY_braille_dots_378: u32 = 0x10028c4;

/// U+28C5 BRAILLE PATTERN DOTS-1378
pub const FCITX_KEY_braille_dots_1378: u32 = 0x10028c5;

/// U+28C6 BRAILLE PATTERN DOTS-2378
pub const FCITX_KEY_braille_dots_2378: u32 = 0x10028c6;

/// U+28C7 BRAILLE PATTERN DOTS-12378
pub const FCITX_KEY_braille_dots_12378: u32 = 0x10028c7;

/// U+28C8 BRAILLE PATTERN DOTS-478
pub const FCITX_KEY_braille_dots_478: u32 = 0x10028c8;

/// U+28C9 BRAILLE PATTERN DOTS-1478
pub const FCITX_KEY_braille_dots_1478: u32 = 0x10028c9;

/// U+28CA BRAILLE PATTERN DOTS-2478
pub const FCITX_KEY_braille_dots_2478: u32 = 0x10028ca;

/// U+28CB BRAILLE PATTERN DOTS-12478
pub const FCITX_KEY_braille_dots_12478: u32 = 0x10028cb;

/// U+28CC BRAILLE PATTERN DOTS-3478
pub const FCITX_KEY_braille_dots_3478: u32 = 0x10028cc;

/// U+28CD BRAILLE PATTERN DOTS-13478
pub const FCITX_KEY_braille_dots_13478: u32 = 0x10028cd;

/// U+28CE BRAILLE PATTERN DOTS-23478
pub const FCITX_KEY_braille_dots_23478: u32 = 0x10028ce;

/// U+28CF BRAILLE PATTERN DOTS-123478
pub const FCITX_KEY_braille_dots_123478: u32 = 0x10028cf;

/// U+28D0 BRAILLE PATTERN DOTS-578
pub const FCITX_KEY_braille_dots_578: u32 = 0x10028d0;

/// U+28D1 BRAILLE PATTERN DOTS-1578
pub const FCITX_KEY_braille_dots_1578: u32 = 0x10028d1;

/// U+28D2 BRAILLE PATTERN DOTS-2578
pub const FCITX_KEY_braille_dots_2578: u32 = 0x10028d2;

/// U+28D3 BRAILLE PATTERN DOTS-12578
pub const FCITX_KEY_braille_dots_12578: u32 = 0x10028d3;

/// U+28D4 BRAILLE PATTERN DOTS-3578
pub const FCITX_KEY_braille_dots_3578: u32 = 0x10028d4;

/// U+28D5 BRAILLE PATTERN DOTS-13578
pub const FCITX_KEY_braille_dots_13578: u32 = 0x10028d5;

/// U+28D6 BRAILLE PATTERN DOTS-23578
pub const FCITX_KEY_braille_dots_23578: u32 = 0x10028d6;

/// U+28D7 BRAILLE PATTERN DOTS-123578
pub const FCITX_KEY_braille_dots_123578: u32 = 0x10028d7;

/// U+28D8 BRAILLE PATTERN DOTS-4578
pub const FCITX_KEY_braille_dots_4578: u32 = 0x10028d8;

/// U+28D9 BRAILLE PATTERN DOTS-14578
pub const FCITX_KEY_braille_dots_14578: u32 = 0x10028d9;

/// U+28DA BRAILLE PATTERN DOTS-24578
pub const FCITX_KEY_braille_dots_24578: u32 = 0x10028da;

/// U+28DB BRAILLE PATTERN DOTS-124578
pub const FCITX_KEY_braille_dots_124578: u32 = 0x10028db;

/// U+28DC BRAILLE PATTERN DOTS-34578
pub const FCITX_KEY_braille_dots_34578: u32 = 0x10028dc;

/// U+28DD BRAILLE PATTERN DOTS-134578
pub const FCITX_KEY_braille_dots_134578: u32 = 0x10028dd;

/// U+28DE BRAILLE PATTERN DOTS-234578
pub const FCITX_KEY_braille_dots_234578: u32 = 0x10028de;

/// U+28DF BRAILLE PATTERN DOTS-1234578
pub const FCITX_KEY_braille_dots_1234578: u32 = 0x10028df;

/// U+28E0 BRAILLE PATTERN DOTS-678
pub const FCITX_KEY_braille_dots_678: u32 = 0x10028e0;

/// U+28E1 BRAILLE PATTERN DOTS-1678
pub const FCITX_KEY_braille_dots_1678: u32 = 0x10028e1;

/// U+28E2 BRAILLE PATTERN DOTS-2678
pub const FCITX_KEY_braille_dots_2678: u32 = 0x10028e2;

/// U+28E3 BRAILLE PATTERN DOTS-12678
pub const FCITX_KEY_braille_dots_12678: u32 = 0x10028e3;

/// U+28E4 BRAILLE PATTERN DOTS-3678
pub const FCITX_KEY_braille_dots_3678: u32 = 0x10028e4;

/// U+28E5 BRAILLE PATTERN DOTS-13678
pub const FCITX_KEY_braille_dots_13678: u32 = 0x10028e5;

/// U+28E6 BRAILLE PATTERN DOTS-23678
pub const FCITX_KEY_braille_dots_23678: u32 = 0x10028e6;

/// U+28E7 BRAILLE PATTERN DOTS-123678
pub const FCITX_KEY_braille_dots_123678: u32 = 0x10028e7;

/// U+28E8 BRAILLE PATTERN DOTS-4678
pub const FCITX_KEY_braille_dots_4678: u32 = 0x10028e8;

/// U+28E9 BRAILLE PATTERN DOTS-14678
pub const FCITX_KEY_braille_dots_14678: u32 = 0x10028e9;

/// U+28EA BRAILLE PATTERN DOTS-24678
pub const FCITX_KEY_braille_dots_24678: u32 = 0x10028ea;

/// U+28EB BRAILLE PATTERN DOTS-124678
pub const FCITX_KEY_braille_dots_124678: u32 = 0x10028eb;

/// U+28EC BRAILLE PATTERN DOTS-34678
pub const FCITX_KEY_braille_dots_34678: u32 = 0x10028ec;

/// U+28ED BRAILLE PATTERN DOTS-134678
pub const FCITX_KEY_braille_dots_134678: u32 = 0x10028ed;

/// U+28EE BRAILLE PATTERN DOTS-234678
pub const FCITX_KEY_braille_dots_234678: u32 = 0x10028ee;

/// U+28EF BRAILLE PATTERN DOTS-1234678
pub const FCITX_KEY_braille_dots_1234678: u32 = 0x10028ef;

/// U+28F0 BRAILLE PATTERN DOTS-5678
pub const FCITX_KEY_braille_dots_5678: u32 = 0x10028f0;

/// U+28F1 BRAILLE PATTERN DOTS-15678
pub const FCITX_KEY_braille_dots_15678: u32 = 0x10028f1;

/// U+28F2 BRAILLE PATTERN DOTS-25678
pub const FCITX_KEY_braille_dots_25678: u32 = 0x10028f2;

/// U+28F3 BRAILLE PATTERN DOTS-125678
pub const FCITX_KEY_braille_dots_125678: u32 = 0x10028f3;

/// U+28F4 BRAILLE PATTERN DOTS-35678
pub const FCITX_KEY_braille_dots_35678: u32 = 0x10028f4;

/// U+28F5 BRAILLE PATTERN DOTS-135678
pub const FCITX_KEY_braille_dots_135678: u32 = 0x10028f5;

/// U+28F6 BRAILLE PATTERN DOTS-235678
pub const FCITX_KEY_braille_dots_235678: u32 = 0x10028f6;

/// U+28F7 BRAILLE PATTERN DOTS-1235678
pub const FCITX_KEY_braille_dots_1235678: u32 = 0x10028f7;

/// U+28F8 BRAILLE PATTERN DOTS-45678
pub const FCITX_KEY_braille_dots_45678: u32 = 0x10028f8;

/// U+28F9 BRAILLE PATTERN DOTS-145678
pub const FCITX_KEY_braille_dots_145678: u32 = 0x10028f9;

/// U+28FA BRAILLE PATTERN DOTS-245678
pub const FCITX_KEY_braille_dots_245678: u32 = 0x10028fa;

/// U+28FB BRAILLE PATTERN DOTS-1245678
pub const FCITX_KEY_braille_dots_1245678: u32 = 0x10028fb;

/// U+28FC BRAILLE PATTERN DOTS-345678
pub const FCITX_KEY_braille_dots_345678: u32 = 0x10028fc;

/// U+28FD BRAILLE PATTERN DOTS-1345678
pub const FCITX_KEY_braille_dots_1345678: u32 = 0x10028fd;

/// U+28FE BRAILLE PATTERN DOTS-2345678
pub const FCITX_KEY_braille_dots_2345678: u32 = 0x10028fe;

/// U+28FF BRAILLE PATTERN DOTS-12345678
pub const FCITX_KEY_braille_dots_12345678: u32 = 0x10028ff;

/// U+0D82 SINHALA SIGN ANUSVARAYA
pub const FCITX_KEY_Sinh_ng: u32 = 0x1000d82;

/// U+0D83 SINHALA SIGN VISARGAYA
pub const FCITX_KEY_Sinh_h2: u32 = 0x1000d83;

/// U+0D85 SINHALA LETTER AYANNA
pub const FCITX_KEY_Sinh_a: u32 = 0x1000d85;

/// U+0D86 SINHALA LETTER AAYANNA
pub const FCITX_KEY_Sinh_aa: u32 = 0x1000d86;

/// U+0D87 SINHALA LETTER AEYANNA
pub const FCITX_KEY_Sinh_ae: u32 = 0x1000d87;

/// U+0D88 SINHALA LETTER AEEYANNA
pub const FCITX_KEY_Sinh_aee: u32 = 0x1000d88;

/// U+0D89 SINHALA LETTER IYANNA
pub const FCITX_KEY_Sinh_i: u32 = 0x1000d89;

/// U+0D8A SINHALA LETTER IIYANNA
pub const FCITX_KEY_Sinh_ii: u32 = 0x1000d8a;

/// U+0D8B SINHALA LETTER UYANNA
pub const FCITX_KEY_Sinh_u: u32 = 0x1000d8b;

/// U+0D8C SINHALA LETTER UUYANNA
pub const FCITX_KEY_Sinh_uu: u32 = 0x1000d8c;

/// U+0D8D SINHALA LETTER IRUYANNA
pub const FCITX_KEY_Sinh_ri: u32 = 0x1000d8d;

/// U+0D8E SINHALA LETTER IRUUYANNA
pub const FCITX_KEY_Sinh_rii: u32 = 0x1000d8e;

/// U+0D8F SINHALA LETTER ILUYANNA
pub const FCITX_KEY_Sinh_lu: u32 = 0x1000d8f;

/// U+0D90 SINHALA LETTER ILUUYANNA
pub const FCITX_KEY_Sinh_luu: u32 = 0x1000d90;

/// U+0D91 SINHALA LETTER EYANNA
pub const FCITX_KEY_Sinh_e: u32 = 0x1000d91;

/// U+0D92 SINHALA LETTER EEYANNA
pub const FCITX_KEY_Sinh_ee: u32 = 0x1000d92;

/// U+0D93 SINHALA LETTER AIYANNA
pub const FCITX_KEY_Sinh_ai: u32 = 0x1000d93;

/// U+0D94 SINHALA LETTER OYANNA
pub const FCITX_KEY_Sinh_o: u32 = 0x1000d94;

/// U+0D95 SINHALA LETTER OOYANNA
pub const FCITX_KEY_Sinh_oo: u32 = 0x1000d95;

/// U+0D96 SINHALA LETTER AUYANNA
pub const FCITX_KEY_Sinh_au: u32 = 0x1000d96;

/// U+0D9A SINHALA LETTER ALPAPRAANA KAYANNA
pub const FCITX_KEY_Sinh_ka: u32 = 0x1000d9a;

/// U+0D9B SINHALA LETTER MAHAAPRAANA KAYANNA
pub const FCITX_KEY_Sinh_kha: u32 = 0x1000d9b;

/// U+0D9C SINHALA LETTER ALPAPRAANA GAYANNA
pub const FCITX_KEY_Sinh_ga: u32 = 0x1000d9c;

/// U+0D9D SINHALA LETTER MAHAAPRAANA GAYANNA
pub const FCITX_KEY_Sinh_gha: u32 = 0x1000d9d;

/// U+0D9E SINHALA LETTER KANTAJA NAASIKYAYA
pub const FCITX_KEY_Sinh_ng2: u32 = 0x1000d9e;

/// U+0D9F SINHALA LETTER SANYAKA GAYANNA
pub const FCITX_KEY_Sinh_nga: u32 = 0x1000d9f;

/// U+0DA0 SINHALA LETTER ALPAPRAANA CAYANNA
pub const FCITX_KEY_Sinh_ca: u32 = 0x1000da0;

/// U+0DA1 SINHALA LETTER MAHAAPRAANA CAYANNA
pub const FCITX_KEY_Sinh_cha: u32 = 0x1000da1;

/// U+0DA2 SINHALA LETTER ALPAPRAANA JAYANNA
pub const FCITX_KEY_Sinh_ja: u32 = 0x1000da2;

/// U+0DA3 SINHALA LETTER MAHAAPRAANA JAYANNA
pub const FCITX_KEY_Sinh_jha: u32 = 0x1000da3;

/// U+0DA4 SINHALA LETTER TAALUJA NAASIKYAYA
pub const FCITX_KEY_Sinh_nya: u32 = 0x1000da4;

/// U+0DA5 SINHALA LETTER TAALUJA SANYOOGA NAAKSIKYAYA
pub const FCITX_KEY_Sinh_jnya: u32 = 0x1000da5;

/// U+0DA6 SINHALA LETTER SANYAKA JAYANNA
pub const FCITX_KEY_Sinh_nja: u32 = 0x1000da6;

/// U+0DA7 SINHALA LETTER ALPAPRAANA TTAYANNA
pub const FCITX_KEY_Sinh_tta: u32 = 0x1000da7;

/// U+0DA8 SINHALA LETTER MAHAAPRAANA TTAYANNA
pub const FCITX_KEY_Sinh_ttha: u32 = 0x1000da8;

/// U+0DA9 SINHALA LETTER ALPAPRAANA DDAYANNA
pub const FCITX_KEY_Sinh_dda: u32 = 0x1000da9;

/// U+0DAA SINHALA LETTER MAHAAPRAANA DDAYANNA
pub const FCITX_KEY_Sinh_ddha: u32 = 0x1000daa;

/// U+0DAB SINHALA LETTER MUURDHAJA NAYANNA
pub const FCITX_KEY_Sinh_nna: u32 = 0x1000dab;

/// U+0DAC SINHALA LETTER SANYAKA DDAYANNA
pub const FCITX_KEY_Sinh_ndda: u32 = 0x1000dac;

/// U+0DAD SINHALA LETTER ALPAPRAANA TAYANNA
pub const FCITX_KEY_Sinh_tha: u32 = 0x1000dad;

/// U+0DAE SINHALA LETTER MAHAAPRAANA TAYANNA
pub const FCITX_KEY_Sinh_thha: u32 = 0x1000dae;

/// U+0DAF SINHALA LETTER ALPAPRAANA DAYANNA
pub const FCITX_KEY_Sinh_dha: u32 = 0x1000daf;

/// U+0DB0 SINHALA LETTER MAHAAPRAANA DAYANNA
pub const FCITX_KEY_Sinh_dhha: u32 = 0x1000db0;

/// U+0DB1 SINHALA LETTER DANTAJA NAYANNA
pub const FCITX_KEY_Sinh_na: u32 = 0x1000db1;

/// U+0DB3 SINHALA LETTER SANYAKA DAYANNA
pub const FCITX_KEY_Sinh_ndha: u32 = 0x1000db3;

/// U+0DB4 SINHALA LETTER ALPAPRAANA PAYANNA
pub const FCITX_KEY_Sinh_pa: u32 = 0x1000db4;

/// U+0DB5 SINHALA LETTER MAHAAPRAANA PAYANNA
pub const FCITX_KEY_Sinh_pha: u32 = 0x1000db5;

/// U+0DB6 SINHALA LETTER ALPAPRAANA BAYANNA
pub const FCITX_KEY_Sinh_ba: u32 = 0x1000db6;

/// U+0DB7 SINHALA LETTER MAHAAPRAANA BAYANNA
pub const FCITX_KEY_Sinh_bha: u32 = 0x1000db7;

/// U+0DB8 SINHALA LETTER MAYANNA
pub const FCITX_KEY_Sinh_ma: u32 = 0x1000db8;

/// U+0DB9 SINHALA LETTER AMBA BAYANNA
pub const FCITX_KEY_Sinh_mba: u32 = 0x1000db9;

/// U+0DBA SINHALA LETTER YAYANNA
pub const FCITX_KEY_Sinh_ya: u32 = 0x1000dba;

/// U+0DBB SINHALA LETTER RAYANNA
pub const FCITX_KEY_Sinh_ra: u32 = 0x1000dbb;

/// U+0DBD SINHALA LETTER DANTAJA LAYANNA
pub const FCITX_KEY_Sinh_la: u32 = 0x1000dbd;

/// U+0DC0 SINHALA LETTER VAYANNA
pub const FCITX_KEY_Sinh_va: u32 = 0x1000dc0;

/// U+0DC1 SINHALA LETTER TAALUJA SAYANNA
pub const FCITX_KEY_Sinh_sha: u32 = 0x1000dc1;

/// U+0DC2 SINHALA LETTER MUURDHAJA SAYANNA
pub const FCITX_KEY_Sinh_ssha: u32 = 0x1000dc2;

/// U+0DC3 SINHALA LETTER DANTAJA SAYANNA
pub const FCITX_KEY_Sinh_sa: u32 = 0x1000dc3;

/// U+0DC4 SINHALA LETTER HAYANNA
pub const FCITX_KEY_Sinh_ha: u32 = 0x1000dc4;

/// U+0DC5 SINHALA LETTER MUURDHAJA LAYANNA
pub const FCITX_KEY_Sinh_lla: u32 = 0x1000dc5;

/// U+0DC6 SINHALA LETTER FAYANNA
pub const FCITX_KEY_Sinh_fa: u32 = 0x1000dc6;

/// U+0DCA SINHALA SIGN AL-LAKUNA
pub const FCITX_KEY_Sinh_al: u32 = 0x1000dca;

/// U+0DCF SINHALA VOWEL SIGN AELA-PILLA
pub const FCITX_KEY_Sinh_aa2: u32 = 0x1000dcf;

/// U+0DD0 SINHALA VOWEL SIGN KETTI AEDA-PILLA
pub const FCITX_KEY_Sinh_ae2: u32 = 0x1000dd0;

/// U+0DD1 SINHALA VOWEL SIGN DIGA AEDA-PILLA
pub const FCITX_KEY_Sinh_aee2: u32 = 0x1000dd1;

/// U+0DD2 SINHALA VOWEL SIGN KETTI IS-PILLA
pub const FCITX_KEY_Sinh_i2: u32 = 0x1000dd2;

/// U+0DD3 SINHALA VOWEL SIGN DIGA IS-PILLA
pub const FCITX_KEY_Sinh_ii2: u32 = 0x1000dd3;

/// U+0DD4 SINHALA VOWEL SIGN KETTI PAA-PILLA
pub const FCITX_KEY_Sinh_u2: u32 = 0x1000dd4;

/// U+0DD6 SINHALA VOWEL SIGN DIGA PAA-PILLA
pub const FCITX_KEY_Sinh_uu2: u32 = 0x1000dd6;

/// U+0DD8 SINHALA VOWEL SIGN GAETTA-PILLA
pub const FCITX_KEY_Sinh_ru2: u32 = 0x1000dd8;

/// U+0DD9 SINHALA VOWEL SIGN KOMBUVA
pub const FCITX_KEY_Sinh_e2: u32 = 0x1000dd9;

/// U+0DDA SINHALA VOWEL SIGN DIGA KOMBUVA
pub const FCITX_KEY_Sinh_ee2: u32 = 0x1000dda;

/// U+0DDB SINHALA VOWEL SIGN KOMBU DEKA
pub const FCITX_KEY_Sinh_ai2: u32 = 0x1000ddb;

/// U+0DDC SINHALA VOWEL SIGN KOMBUVA HAA AELA-PILLA
pub const FCITX_KEY_Sinh_o2: u32 = 0x1000ddc;

/// U+0DDD SINHALA VOWEL SIGN KOMBUVA HAA DIGA AELA-PILLA
pub const FCITX_KEY_Sinh_oo2: u32 = 0x1000ddd;

/// U+0DDE SINHALA VOWEL SIGN KOMBUVA HAA GAYANUKITTA
pub const FCITX_KEY_Sinh_au2: u32 = 0x1000dde;

/// U+0DDF SINHALA VOWEL SIGN GAYANUKITTA
pub const FCITX_KEY_Sinh_lu2: u32 = 0x1000ddf;

/// U+0DF2 SINHALA VOWEL SIGN DIGA GAETTA-PILLA
pub const FCITX_KEY_Sinh_ruu2: u32 = 0x1000df2;

/// U+0DF3 SINHALA VOWEL SIGN DIGA GAYANUKITTA
pub const FCITX_KEY_Sinh_luu2: u32 = 0x1000df3;

/// U+0DF4 SINHALA PUNCTUATION KUNDDALIYA
pub const FCITX_KEY_Sinh_kunddaliya: u32 = 0x1000df4;

/// Mode Switch Lock
pub const FCITX_KEY_ModeLock: u32 = 0x1008ff01;

/// Monitor/panel brightness
pub const FCITX_KEY_MonBrightnessUp: u32 = 0x1008ff02;

/// Monitor/panel brightness
pub const FCITX_KEY_MonBrightnessDown: u32 = 0x1008ff03;

/// Keyboards may be lit
pub const FCITX_KEY_KbdLightOnOff: u32 = 0x1008ff04;

/// Keyboards may be lit
pub const FCITX_KEY_KbdBrightnessUp: u32 = 0x1008ff05;

/// Keyboards may be lit
pub const FCITX_KEY_KbdBrightnessDown: u32 = 0x1008ff06;

/// Monitor/panel brightness
pub const FCITX_KEY_MonBrightnessCycle: u32 = 0x1008ff07;

/// System into standby mode
pub const FCITX_KEY_Standby: u32 = 0x1008ff10;

/// Volume control down
pub const FCITX_KEY_AudioLowerVolume: u32 = 0x1008ff11;

/// Mute sound from the system
pub const FCITX_KEY_AudioMute: u32 = 0x1008ff12;

/// Volume control up
pub const FCITX_KEY_AudioRaiseVolume: u32 = 0x1008ff13;

/// Start playing of audio >
pub const FCITX_KEY_AudioPlay: u32 = 0x1008ff14;

/// Stop playing audio
pub const FCITX_KEY_AudioStop: u32 = 0x1008ff15;

/// Previous track
pub const FCITX_KEY_AudioPrev: u32 = 0x1008ff16;

/// Next track
pub const FCITX_KEY_AudioNext: u32 = 0x1008ff17;

/// Display user's home page
pub const FCITX_KEY_HomePage: u32 = 0x1008ff18;

/// Invoke user's mail program
pub const FCITX_KEY_Mail: u32 = 0x1008ff19;

/// Start application
pub const FCITX_KEY_Start: u32 = 0x1008ff1a;

/// Search
pub const FCITX_KEY_Search: u32 = 0x1008ff1b;

/// Record audio application
pub const FCITX_KEY_AudioRecord: u32 = 0x1008ff1c;

/// Invoke calculator program
pub const FCITX_KEY_Calculator: u32 = 0x1008ff1d;

/// Invoke Memo taking program
pub const FCITX_KEY_Memo: u32 = 0x1008ff1e;

/// Invoke To Do List program
pub const FCITX_KEY_ToDoList: u32 = 0x1008ff1f;

/// Invoke Calendar program
pub const FCITX_KEY_Calendar: u32 = 0x1008ff20;

/// Deep sleep the system
pub const FCITX_KEY_PowerDown: u32 = 0x1008ff21;

/// Adjust screen contrast
pub const FCITX_KEY_ContrastAdjust: u32 = 0x1008ff22;

/// Rocker switches exist up
pub const FCITX_KEY_RockerUp: u32 = 0x1008ff23;

/// and down
pub const FCITX_KEY_RockerDown: u32 = 0x1008ff24;

/// and let you press them
pub const FCITX_KEY_RockerEnter: u32 = 0x1008ff25;

/// Like back on a browser
pub const FCITX_KEY_Back: u32 = 0x1008ff26;

/// Like forward on a browser
pub const FCITX_KEY_Forward: u32 = 0x1008ff27;

/// Stop current operation
pub const FCITX_KEY_Stop: u32 = 0x1008ff28;

/// Refresh the page
pub const FCITX_KEY_Refresh: u32 = 0x1008ff29;

/// Power off system entirely
pub const FCITX_KEY_PowerOff: u32 = 0x1008ff2a;

/// Wake up system from sleep
pub const FCITX_KEY_WakeUp: u32 = 0x1008ff2b;

/// Eject device (e.g. DVD)
pub const FCITX_KEY_Eject: u32 = 0x1008ff2c;

/// Invoke screensaver
pub const FCITX_KEY_ScreenSaver: u32 = 0x1008ff2d;

/// Invoke web browser
pub const FCITX_KEY_WWW: u32 = 0x1008ff2e;

/// Put system to sleep
pub const FCITX_KEY_Sleep: u32 = 0x1008ff2f;

/// Show favorite locations
pub const FCITX_KEY_Favorites: u32 = 0x1008ff30;

/// Pause audio playing
pub const FCITX_KEY_AudioPause: u32 = 0x1008ff31;

/// Launch media collection app
pub const FCITX_KEY_AudioMedia: u32 = 0x1008ff32;

/// Display "My Computer" window
pub const FCITX_KEY_MyComputer: u32 = 0x1008ff33;

/// Display vendor home web site
pub const FCITX_KEY_VendorHome: u32 = 0x1008ff34;

/// Light bulb keys exist
pub const FCITX_KEY_LightBulb: u32 = 0x1008ff35;

/// Display shopping web site
pub const FCITX_KEY_Shop: u32 = 0x1008ff36;

/// Show history of web surfing
pub const FCITX_KEY_History: u32 = 0x1008ff37;

/// Open selected URL
pub const FCITX_KEY_OpenURL: u32 = 0x1008ff38;

/// Add URL to favorites list
pub const FCITX_KEY_AddFavorite: u32 = 0x1008ff39;

/// Show "hot" links
pub const FCITX_KEY_HotLinks: u32 = 0x1008ff3a;

/// Invoke brightness adj. UI
pub const FCITX_KEY_BrightnessAdjust: u32 = 0x1008ff3b;

/// Display financial site
pub const FCITX_KEY_Finance: u32 = 0x1008ff3c;

/// Display user's community
pub const FCITX_KEY_Community: u32 = 0x1008ff3d;

/// "rewind" audio track
pub const FCITX_KEY_AudioRewind: u32 = 0x1008ff3e;

/// ???
pub const FCITX_KEY_BackForward: u32 = 0x1008ff3f;

/// Launch Application
pub const FCITX_KEY_Launch0: u32 = 0x1008ff40;

/// Launch Application
pub const FCITX_KEY_Launch1: u32 = 0x1008ff41;

/// Launch Application
pub const FCITX_KEY_Launch2: u32 = 0x1008ff42;

/// Launch Application
pub const FCITX_KEY_Launch3: u32 = 0x1008ff43;

/// Launch Application
pub const FCITX_KEY_Launch4: u32 = 0x1008ff44;

/// Launch Application
pub const FCITX_KEY_Launch5: u32 = 0x1008ff45;

/// Launch Application
pub const FCITX_KEY_Launch6: u32 = 0x1008ff46;

/// Launch Application
pub const FCITX_KEY_Launch7: u32 = 0x1008ff47;

/// Launch Application
pub const FCITX_KEY_Launch8: u32 = 0x1008ff48;

/// Launch Application
pub const FCITX_KEY_Launch9: u32 = 0x1008ff49;

/// Launch Application
pub const FCITX_KEY_LaunchA: u32 = 0x1008ff4a;

/// Launch Application
pub const FCITX_KEY_LaunchB: u32 = 0x1008ff4b;

/// Launch Application
pub const FCITX_KEY_LaunchC: u32 = 0x1008ff4c;

/// Launch Application
pub const FCITX_KEY_LaunchD: u32 = 0x1008ff4d;

/// Launch Application
pub const FCITX_KEY_LaunchE: u32 = 0x1008ff4e;

/// Launch Application
pub const FCITX_KEY_LaunchF: u32 = 0x1008ff4f;

/// switch to application, left
pub const FCITX_KEY_ApplicationLeft: u32 = 0x1008ff50;

/// switch to application, right
pub const FCITX_KEY_ApplicationRight: u32 = 0x1008ff51;

/// Launch bookreader
pub const FCITX_KEY_Book: u32 = 0x1008ff52;

/// Launch CD/DVD player
pub const FCITX_KEY_CD: u32 = 0x1008ff53;

/// Alias for XF86CD
pub const FCITX_KEY_MediaSelectCD: u32 = 0x1008ff53;

/// Launch Calculater
pub const FCITX_KEY_Calculater: u32 = 0x1008ff54;

/// Clear window, screen
pub const FCITX_KEY_WindowClear: u32 = 0x1008ff55;

/// Close window
pub const FCITX_KEY_Close: u32 = 0x1008ff56;

/// Copy selection
pub const FCITX_KEY_Copy: u32 = 0x1008ff57;

/// Cut selection
pub const FCITX_KEY_Cut: u32 = 0x1008ff58;

/// Output switch key
pub const FCITX_KEY_Display: u32 = 0x1008ff59;

/// Launch DOS (emulation)
pub const FCITX_KEY_DOS: u32 = 0x1008ff5a;

/// Open documents window
pub const FCITX_KEY_Documents: u32 = 0x1008ff5b;

/// Launch spread sheet
pub const FCITX_KEY_Excel: u32 = 0x1008ff5c;

/// Launch file explorer
pub const FCITX_KEY_Explorer: u32 = 0x1008ff5d;

/// Launch game
pub const FCITX_KEY_Game: u32 = 0x1008ff5e;

/// Go to URL
pub const FCITX_KEY_Go: u32 = 0x1008ff5f;

/// Logitech iTouch- don't use
pub const FCITX_KEY_iTouch: u32 = 0x1008ff60;

/// Log off system
pub const FCITX_KEY_LogOff: u32 = 0x1008ff61;

/// ??
pub const FCITX_KEY_Market: u32 = 0x1008ff62;

/// enter meeting in calendar
pub const FCITX_KEY_Meeting: u32 = 0x1008ff63;

/// distinguish keyboard from PB
pub const FCITX_KEY_MenuKB: u32 = 0x1008ff65;

/// distinguish PB from keyboard
pub const FCITX_KEY_MenuPB: u32 = 0x1008ff66;

/// Favourites
pub const FCITX_KEY_MySites: u32 = 0x1008ff67;

/// New (folder, document...
pub const FCITX_KEY_New: u32 = 0x1008ff68;

/// News
pub const FCITX_KEY_News: u32 = 0x1008ff69;

/// Office home (old Staroffice)
pub const FCITX_KEY_OfficeHome: u32 = 0x1008ff6a;

/// Open
pub const FCITX_KEY_Open: u32 = 0x1008ff6b;

/// ??
pub const FCITX_KEY_Option: u32 = 0x1008ff6c;

/// Paste
pub const FCITX_KEY_Paste: u32 = 0x1008ff6d;

/// Launch phone; dial number
pub const FCITX_KEY_Phone: u32 = 0x1008ff6e;

/// Compaq's Q - don't use
pub const FCITX_KEY_Compaq_Q: u32 = 0x1008ff70;

/// Reply e.g., mail
pub const FCITX_KEY_Reply: u32 = 0x1008ff72;

/// Reload web page, file, etc.
pub const FCITX_KEY_Reload: u32 = 0x1008ff73;

/// Rotate windows e.g. xrandr
pub const FCITX_KEY_RotateWindows: u32 = 0x1008ff74;

/// don't use
pub const FCITX_KEY_RotationPB: u32 = 0x1008ff75;

/// don't use
pub const FCITX_KEY_RotationKB: u32 = 0x1008ff76;

/// Save (file, document, state
pub const FCITX_KEY_Save: u32 = 0x1008ff77;

/// Scroll window/contents up
pub const FCITX_KEY_ScrollUp: u32 = 0x1008ff78;

/// Scroll window/contentd down
pub const FCITX_KEY_ScrollDown: u32 = 0x1008ff79;

/// Use XKB mousekeys instead
pub const FCITX_KEY_ScrollClick: u32 = 0x1008ff7a;

/// Send mail, file, object
pub const FCITX_KEY_Send: u32 = 0x1008ff7b;

/// Spell checker
pub const FCITX_KEY_Spell: u32 = 0x1008ff7c;

/// Split window or screen
pub const FCITX_KEY_SplitScreen: u32 = 0x1008ff7d;

/// Get support (??)
pub const FCITX_KEY_Support: u32 = 0x1008ff7e;

/// Show tasks
pub const FCITX_KEY_TaskPane: u32 = 0x1008ff7f;

/// Launch terminal emulator
pub const FCITX_KEY_Terminal: u32 = 0x1008ff80;

/// toolbox of desktop/app.
pub const FCITX_KEY_Tools: u32 = 0x1008ff81;

/// ??
pub const FCITX_KEY_Travel: u32 = 0x1008ff82;

/// ??
pub const FCITX_KEY_UserPB: u32 = 0x1008ff84;

/// ??
pub const FCITX_KEY_User1KB: u32 = 0x1008ff85;

/// ??
pub const FCITX_KEY_User2KB: u32 = 0x1008ff86;

/// Launch video player
pub const FCITX_KEY_Video: u32 = 0x1008ff87;

/// button from a mouse wheel
pub const FCITX_KEY_WheelButton: u32 = 0x1008ff88;

/// Launch word processor
pub const FCITX_KEY_Word: u32 = 0x1008ff89;

pub const FCITX_KEY_Xfer: u32 = 0x1008ff8a;

/// zoom in view, map, etc.
pub const FCITX_KEY_ZoomIn: u32 = 0x1008ff8b;

/// zoom out view, map, etc.
pub const FCITX_KEY_ZoomOut: u32 = 0x1008ff8c;

/// mark yourself as away
pub const FCITX_KEY_Away: u32 = 0x1008ff8d;

/// as in instant messaging
pub const FCITX_KEY_Messenger: u32 = 0x1008ff8e;

/// Launch web camera app.
pub const FCITX_KEY_WebCam: u32 = 0x1008ff8f;

/// Forward in mail
pub const FCITX_KEY_MailForward: u32 = 0x1008ff90;

/// Show pictures
pub const FCITX_KEY_Pictures: u32 = 0x1008ff91;

/// Launch music application
pub const FCITX_KEY_Music: u32 = 0x1008ff92;

/// Display battery information
pub const FCITX_KEY_Battery: u32 = 0x1008ff93;

/// Enable/disable Bluetooth
pub const FCITX_KEY_Bluetooth: u32 = 0x1008ff94;

/// Enable/disable WLAN
pub const FCITX_KEY_WLAN: u32 = 0x1008ff95;

/// fast-forward audio track
pub const FCITX_KEY_AudioForward: u32 = 0x1008ff97;

/// toggle repeat mode
pub const FCITX_KEY_AudioRepeat: u32 = 0x1008ff98;

/// toggle shuffle mode
pub const FCITX_KEY_AudioRandomPlay: u32 = 0x1008ff99;

/// cycle through subtitle
pub const FCITX_KEY_Subtitle: u32 = 0x1008ff9a;

/// cycle through audio tracks
pub const FCITX_KEY_AudioCycleTrack: u32 = 0x1008ff9b;

/// cycle through angles
pub const FCITX_KEY_CycleAngle: u32 = 0x1008ff9c;

/// video: go one frame back
pub const FCITX_KEY_FrameBack: u32 = 0x1008ff9d;

/// video: go one frame forward
pub const FCITX_KEY_FrameForward: u32 = 0x1008ff9e;

/// display, or shows an entry for time seeking
pub const FCITX_KEY_Time: u32 = 0x1008ff9f;

/// Select button on joypads and remotes
pub const FCITX_KEY_SelectButton: u32 = 0x1008ffa0;

/// Show a view options/properties
pub const FCITX_KEY_View: u32 = 0x1008ffa1;

/// Go to a top-level menu in a video
pub const FCITX_KEY_TopMenu: u32 = 0x1008ffa2;

/// Red button
pub const FCITX_KEY_Red: u32 = 0x1008ffa3;

/// Green button
pub const FCITX_KEY_Green: u32 = 0x1008ffa4;

/// Yellow button
pub const FCITX_KEY_Yellow: u32 = 0x1008ffa5;

/// Blue button
pub const FCITX_KEY_Blue: u32 = 0x1008ffa6;

/// Sleep to RAM
pub const FCITX_KEY_Suspend: u32 = 0x1008ffa7;

/// Sleep to disk
pub const FCITX_KEY_Hibernate: u32 = 0x1008ffa8;

/// Toggle between touchpad/trackstick
pub const FCITX_KEY_TouchpadToggle: u32 = 0x1008ffa9;

/// The touchpad got switched on
pub const FCITX_KEY_TouchpadOn: u32 = 0x1008ffb0;

/// The touchpad got switched off
pub const FCITX_KEY_TouchpadOff: u32 = 0x1008ffb1;

/// Mute the Mic from the system
pub const FCITX_KEY_AudioMicMute: u32 = 0x1008ffb2;

/// User defined keyboard related action
pub const FCITX_KEY_Keyboard: u32 = 0x1008ffb3;

/// Toggle WWAN (LTE, UMTS, etc.) radio
pub const FCITX_KEY_WWAN: u32 = 0x1008ffb4;

/// Toggle radios on/off
pub const FCITX_KEY_RFKill: u32 = 0x1008ffb5;

/// Select equalizer preset, e.g. theatre-mode
pub const FCITX_KEY_AudioPreset: u32 = 0x1008ffb6;

/// Toggle screen rotation lock on/off
pub const FCITX_KEY_RotationLockToggle: u32 = 0x1008ffb7;

/// Toggle fullscreen
pub const FCITX_KEY_FullScreen: u32 = 0x1008ffb8;

pub const FCITX_KEY_Switch_VT_1: u32 = 0x1008fe01;

pub const FCITX_KEY_Switch_VT_2: u32 = 0x1008fe02;

pub const FCITX_KEY_Switch_VT_3: u32 = 0x1008fe03;

pub const FCITX_KEY_Switch_VT_4: u32 = 0x1008fe04;

pub const FCITX_KEY_Switch_VT_5: u32 = 0x1008fe05;

pub const FCITX_KEY_Switch_VT_6: u32 = 0x1008fe06;

pub const FCITX_KEY_Switch_VT_7: u32 = 0x1008fe07;

pub const FCITX_KEY_Switch_VT_8: u32 = 0x1008fe08;

pub const FCITX_KEY_Switch_VT_9: u32 = 0x1008fe09;

pub const FCITX_KEY_Switch_VT_10: u32 = 0x1008fe0a;

pub const FCITX_KEY_Switch_VT_11: u32 = 0x1008fe0b;

pub const FCITX_KEY_Switch_VT_12: u32 = 0x1008fe0c;

/// force ungrab
pub const FCITX_KEY_Ungrab: u32 = 0x1008fe20;

/// kill application with grab
pub const FCITX_KEY_ClearGrab: u32 = 0x1008fe21;

/// next video mode available
pub const FCITX_KEY_Next_VMode: u32 = 0x1008fe22;

/// prev. video mode available
pub const FCITX_KEY_Prev_VMode: u32 = 0x1008fe23;

/// print window tree to log
pub const FCITX_KEY_LogWindowTree: u32 = 0x1008fe24;

/// print all active grabs to log
pub const FCITX_KEY_LogGrabInfo: u32 = 0x1008fe25;

/// KEY_PLAYPAUSE
pub const FCITX_KEY_MediaPlayPause: u32 = 0x100810a4;

/// KEY_EXIT
pub const FCITX_KEY_Exit: u32 = 0x100810ae;

/// KEY_BASSBOOST
pub const FCITX_KEY_AudioBassBoost: u32 = 0x100810d1;

/// KEY_SPORT
pub const FCITX_KEY_Sport: u32 = 0x100810dc;

/// Deprecated alias for XF86MonBrightnessAuto
pub const FCITX_KEY_BrightnessAuto: u32 = 0x100810f4;

/// v3.16   KEY_BRIGHTNESS_AUTO
pub const FCITX_KEY_MonBrightnessAuto: u32 = 0x100810f4;

/// v2.6.23 KEY_DISPLAY_OFF
pub const FCITX_KEY_DisplayOff: u32 = 0x100810f5;

/// v2.5.26 KEY_OK
pub const FCITX_KEY_OK: u32 = 0x10081160;

/// v2.5.26 KEY_GOTO
pub const FCITX_KEY_GoTo: u32 = 0x10081162;

/// v2.5.26 KEY_INFO
pub const FCITX_KEY_Info: u32 = 0x10081166;

/// v2.5.26 KEY_VENDOR
pub const FCITX_KEY_VendorLogo: u32 = 0x10081168;

/// v2.5.26 KEY_PROGRAM
pub const FCITX_KEY_MediaSelectProgramGuide: u32 = 0x1008116a;

/// v2.5.26 KEY_PVR
pub const FCITX_KEY_MediaSelectHome: u32 = 0x1008116e;

/// v2.5.26 KEY_LANGUAGE
pub const FCITX_KEY_MediaLanguageMenu: u32 = 0x10081170;

/// v2.5.26 KEY_TITLE
pub const FCITX_KEY_MediaTitleMenu: u32 = 0x10081171;

/// v2.5.26 KEY_MODE
pub const FCITX_KEY_AudioChannelMode: u32 = 0x10081175;

/// v5.1    KEY_ASPECT_RATIO
pub const FCITX_KEY_AspectRatio: u32 = 0x10081177;

/// v2.5.26 KEY_PC
pub const FCITX_KEY_MediaSelectPC: u32 = 0x10081178;

/// v2.5.26 KEY_TV
pub const FCITX_KEY_MediaSelectTV: u32 = 0x10081179;

/// v2.5.26 KEY_TV2
pub const FCITX_KEY_MediaSelectCable: u32 = 0x1008117a;

/// v2.5.26 KEY_VCR
pub const FCITX_KEY_MediaSelectVCR: u32 = 0x1008117b;

/// v2.5.26 KEY_VCR2
pub const FCITX_KEY_MediaSelectVCRPlus: u32 = 0x1008117c;

/// v2.5.26 KEY_SAT
pub const FCITX_KEY_MediaSelectSatellite: u32 = 0x1008117d;

/// v2.5.26 KEY_TAPE
pub const FCITX_KEY_MediaSelectTape: u32 = 0x10081180;

/// v2.5.26 KEY_RADIO
pub const FCITX_KEY_MediaSelectRadio: u32 = 0x10081181;

/// v2.5.26 KEY_TUNER
pub const FCITX_KEY_MediaSelectTuner: u32 = 0x10081182;

/// v2.5.26 KEY_PLAYER
pub const FCITX_KEY_MediaPlayer: u32 = 0x10081183;

/// v2.5.26 KEY_TEXT
pub const FCITX_KEY_MediaSelectTeletext: u32 = 0x10081184;

/// v2.5.26 KEY_DVD
pub const FCITX_KEY_DVD: u32 = 0x10081185;

/// Alias for XF86DVD
pub const FCITX_KEY_MediaSelectDVD: u32 = 0x10081185;

/// v2.5.26 KEY_AUX
pub const FCITX_KEY_MediaSelectAuxiliary: u32 = 0x10081186;

/// v2.5.26 KEY_AUDIO
pub const FCITX_KEY_Audio: u32 = 0x10081188;

/// v2.5.26 KEY_CHANNELUP
pub const FCITX_KEY_ChannelUp: u32 = 0x10081192;

/// v2.5.26 KEY_CHANNELDOWN
pub const FCITX_KEY_ChannelDown: u32 = 0x10081193;

/// v2.5.26 KEY_SLOW
pub const FCITX_KEY_MediaPlaySlow: u32 = 0x10081199;

/// v2.5.26 KEY_BREAK
pub const FCITX_KEY_MediaBreak: u32 = 0x1008119b;

/// v2.5.26 KEY_DIGITS
pub const FCITX_KEY_NumberEntryMode: u32 = 0x1008119d;

/// v2.6.20 KEY_VIDEOPHONE
pub const FCITX_KEY_VideoPhone: u32 = 0x100811a0;

/// v2.6.20 KEY_ZOOMRESET
pub const FCITX_KEY_ZoomReset: u32 = 0x100811a4;

/// v2.6.20 KEY_EDITOR
pub const FCITX_KEY_Editor: u32 = 0x100811a6;

/// v2.6.20 KEY_GRAPHICSEDITOR
pub const FCITX_KEY_GraphicsEditor: u32 = 0x100811a8;

/// v2.6.20 KEY_PRESENTATION
pub const FCITX_KEY_Presentation: u32 = 0x100811a9;

/// v2.6.20 KEY_DATABASE
pub const FCITX_KEY_Database: u32 = 0x100811aa;

/// v2.6.20 KEY_VOICEMAIL
pub const FCITX_KEY_Voicemail: u32 = 0x100811ac;

/// v2.6.20 KEY_ADDRESSBOOK
pub const FCITX_KEY_Addressbook: u32 = 0x100811ad;

/// v2.6.20 KEY_DISPLAYTOGGLE
pub const FCITX_KEY_DisplayToggle: u32 = 0x100811af;

/// v2.6.24 KEY_SPELLCHECK
pub const FCITX_KEY_SpellCheck: u32 = 0x100811b0;

/// v2.6.24 KEY_CONTEXT_MENU
pub const FCITX_KEY_ContextMenu: u32 = 0x100811b6;

/// v2.6.26 KEY_MEDIA_REPEAT
pub const FCITX_KEY_MediaRepeat: u32 = 0x100811b7;

/// v2.6.38 KEY_10CHANNELSUP
pub const FCITX_KEY_10ChannelsUp: u32 = 0x100811b8;

/// v2.6.38 KEY_10CHANNELSDOWN
pub const FCITX_KEY_10ChannelsDown: u32 = 0x100811b9;

/// v2.6.39 KEY_IMAGES
pub const FCITX_KEY_Images: u32 = 0x100811ba;

/// v5.10   KEY_NOTIFICATION_CENTER
pub const FCITX_KEY_NotificationCenter: u32 = 0x100811bc;

/// v5.10   KEY_PICKUP_PHONE
pub const FCITX_KEY_PickupPhone: u32 = 0x100811bd;

/// v5.10   KEY_HANGUP_PHONE
pub const FCITX_KEY_HangupPhone: u32 = 0x100811be;

/// v6.14   KEY_LINK_PHONE
pub const FCITX_KEY_LinkPhone: u32 = 0x100811bf;

/// KEY_FN
pub const FCITX_KEY_Fn: u32 = 0x100811d0;

/// KEY_FN_ESC
pub const FCITX_KEY_Fn_Esc: u32 = 0x100811d1;

/// KEY_FN_F1
pub const FCITX_KEY_Fn_F1: u32 = 0x100811d2;

/// KEY_FN_F2
pub const FCITX_KEY_Fn_F2: u32 = 0x100811d3;

/// KEY_FN_F3
pub const FCITX_KEY_Fn_F3: u32 = 0x100811d4;

/// KEY_FN_F4
pub const FCITX_KEY_Fn_F4: u32 = 0x100811d5;

/// KEY_FN_F5
pub const FCITX_KEY_Fn_F5: u32 = 0x100811d6;

/// KEY_FN_F6
pub const FCITX_KEY_Fn_F6: u32 = 0x100811d7;

/// KEY_FN_F7
pub const FCITX_KEY_Fn_F7: u32 = 0x100811d8;

/// KEY_FN_F8
pub const FCITX_KEY_Fn_F8: u32 = 0x100811d9;

/// KEY_FN_F9
pub const FCITX_KEY_Fn_F9: u32 = 0x100811da;

/// KEY_FN_F10
pub const FCITX_KEY_Fn_F10: u32 = 0x100811db;

/// KEY_FN_F11
pub const FCITX_KEY_Fn_F11: u32 = 0x100811dc;

/// KEY_FN_F12
pub const FCITX_KEY_Fn_F12: u32 = 0x100811dd;

/// KEY_FN_1
pub const FCITX_KEY_Fn_1: u32 = 0x100811de;

/// KEY_FN_2
pub const FCITX_KEY_Fn_2: u32 = 0x100811df;

/// KEY_FN_D
pub const FCITX_KEY_Fn_D: u32 = 0x100811e0;

/// KEY_FN_E
pub const FCITX_KEY_Fn_E: u32 = 0x100811e1;

/// KEY_FN_F
pub const FCITX_KEY_Fn_F: u32 = 0x100811e2;

/// KEY_FN_S
pub const FCITX_KEY_Fn_S: u32 = 0x100811e3;

/// KEY_FN_B
pub const FCITX_KEY_Fn_B: u32 = 0x100811e4;

/// v5.10   KEY_FN_RIGHT_SHIFT
pub const FCITX_KEY_FnRightShift: u32 = 0x100811e5;

/// v2.6.28 KEY_NUMERIC_0
pub const FCITX_KEY_Numeric0: u32 = 0x10081200;

/// v2.6.28 KEY_NUMERIC_1
pub const FCITX_KEY_Numeric1: u32 = 0x10081201;

/// v2.6.28 KEY_NUMERIC_2
pub const FCITX_KEY_Numeric2: u32 = 0x10081202;

/// v2.6.28 KEY_NUMERIC_3
pub const FCITX_KEY_Numeric3: u32 = 0x10081203;

/// v2.6.28 KEY_NUMERIC_4
pub const FCITX_KEY_Numeric4: u32 = 0x10081204;

/// v2.6.28 KEY_NUMERIC_5
pub const FCITX_KEY_Numeric5: u32 = 0x10081205;

/// v2.6.28 KEY_NUMERIC_6
pub const FCITX_KEY_Numeric6: u32 = 0x10081206;

/// v2.6.28 KEY_NUMERIC_7
pub const FCITX_KEY_Numeric7: u32 = 0x10081207;

/// v2.6.28 KEY_NUMERIC_8
pub const FCITX_KEY_Numeric8: u32 = 0x10081208;

/// v2.6.28 KEY_NUMERIC_9
pub const FCITX_KEY_Numeric9: u32 = 0x10081209;

/// v2.6.28 KEY_NUMERIC_STAR
pub const FCITX_KEY_NumericStar: u32 = 0x1008120a;

/// v2.6.28 KEY_NUMERIC_POUND
pub const FCITX_KEY_NumericPound: u32 = 0x1008120b;

/// v4.1    KEY_NUMERIC_A
pub const FCITX_KEY_NumericA: u32 = 0x1008120c;

/// v4.1    KEY_NUMERIC_B
pub const FCITX_KEY_NumericB: u32 = 0x1008120d;

/// v4.1    KEY_NUMERIC_C
pub const FCITX_KEY_NumericC: u32 = 0x1008120e;

/// v4.1    KEY_NUMERIC_D
pub const FCITX_KEY_NumericD: u32 = 0x1008120f;

/// v2.6.33 KEY_CAMERA_FOCUS
pub const FCITX_KEY_CameraFocus: u32 = 0x10081210;

/// v2.6.34 KEY_WPS_BUTTON
pub const FCITX_KEY_WPSButton: u32 = 0x10081211;

/// v2.6.39 KEY_CAMERA_ZOOMIN
pub const FCITX_KEY_CameraZoomIn: u32 = 0x10081215;

/// v2.6.39 KEY_CAMERA_ZOOMOUT
pub const FCITX_KEY_CameraZoomOut: u32 = 0x10081216;

/// v2.6.39 KEY_CAMERA_UP
pub const FCITX_KEY_CameraUp: u32 = 0x10081217;

/// v2.6.39 KEY_CAMERA_DOWN
pub const FCITX_KEY_CameraDown: u32 = 0x10081218;

/// v2.6.39 KEY_CAMERA_LEFT
pub const FCITX_KEY_CameraLeft: u32 = 0x10081219;

/// v2.6.39 KEY_CAMERA_RIGHT
pub const FCITX_KEY_CameraRight: u32 = 0x1008121a;

/// v3.10   KEY_ATTENDANT_ON
pub const FCITX_KEY_AttendantOn: u32 = 0x1008121b;

/// v3.10   KEY_ATTENDANT_OFF
pub const FCITX_KEY_AttendantOff: u32 = 0x1008121c;

/// v3.10   KEY_ATTENDANT_TOGGLE
pub const FCITX_KEY_AttendantToggle: u32 = 0x1008121d;

/// v3.10   KEY_LIGHTS_TOGGLE
pub const FCITX_KEY_LightsToggle: u32 = 0x1008121e;

/// v3.13   KEY_ALS_TOGGLE
pub const FCITX_KEY_ALSToggle: u32 = 0x10081230;

/// v6.9    KEY_REFRESH_RATE_TOGGLE
pub const FCITX_KEY_RefreshRateToggle: u32 = 0x10081232;

/// v3.16   KEY_BUTTONCONFIG
pub const FCITX_KEY_Buttonconfig: u32 = 0x10081240;

/// v3.16   KEY_TASKMANAGER
pub const FCITX_KEY_Taskmanager: u32 = 0x10081241;

/// v3.16   KEY_JOURNAL
pub const FCITX_KEY_Journal: u32 = 0x10081242;

/// v3.16   KEY_CONTROLPANEL
pub const FCITX_KEY_ControlPanel: u32 = 0x10081243;

/// v3.16   KEY_APPSELECT
pub const FCITX_KEY_AppSelect: u32 = 0x10081244;

/// v3.16   KEY_SCREENSAVER
pub const FCITX_KEY_Screensaver: u32 = 0x10081245;

/// v3.16   KEY_VOICECOMMAND
pub const FCITX_KEY_VoiceCommand: u32 = 0x10081246;

/// v4.13   KEY_ASSISTANT
pub const FCITX_KEY_Assistant: u32 = 0x10081247;

/// v5.13   KEY_EMOJI_PICKER
pub const FCITX_KEY_EmojiPicker: u32 = 0x10081249;

/// v5.17   KEY_DICTATE
pub const FCITX_KEY_Dictate: u32 = 0x1008124a;

/// v6.2    KEY_CAMERA_ACCESS_ENABLE
pub const FCITX_KEY_CameraAccessEnable: u32 = 0x1008124b;

/// v6.2    KEY_CAMERA_ACCESS_DISABLE
pub const FCITX_KEY_CameraAccessDisable: u32 = 0x1008124c;

/// v6.2    KEY_CAMERA_ACCESS_TOGGLE
pub const FCITX_KEY_CameraAccessToggle: u32 = 0x1008124d;

/// v6.10   KEY_ACCESSIBILITY
pub const FCITX_KEY_Accessibility: u32 = 0x1008124e;

/// v6.10   KEY_DO_NOT_DISTURB
pub const FCITX_KEY_DoNotDisturb: u32 = 0x1008124f;

/// v3.16   KEY_BRIGHTNESS_MIN
pub const FCITX_KEY_BrightnessMin: u32 = 0x10081250;

/// v3.16   KEY_BRIGHTNESS_MAX
pub const FCITX_KEY_BrightnessMax: u32 = 0x10081251;

/// v3.18   KEY_KBDINPUTASSIST_PREV
pub const FCITX_KEY_KbdInputAssistPrev: u32 = 0x10081260;

/// v3.18   KEY_KBDINPUTASSIST_NEXT
pub const FCITX_KEY_KbdInputAssistNext: u32 = 0x10081261;

/// v3.18   KEY_KBDINPUTASSIST_PREVGROUP
pub const FCITX_KEY_KbdInputAssistPrevgroup: u32 = 0x10081262;

/// v3.18   KEY_KBDINPUTASSIST_NEXTGROUP
pub const FCITX_KEY_KbdInputAssistNextgroup: u32 = 0x10081263;

/// v3.18   KEY_KBDINPUTASSIST_ACCEPT
pub const FCITX_KEY_KbdInputAssistAccept: u32 = 0x10081264;

/// v3.18   KEY_KBDINPUTASSIST_CANCEL
pub const FCITX_KEY_KbdInputAssistCancel: u32 = 0x10081265;

/// v4.7    KEY_RIGHT_UP
pub const FCITX_KEY_RightUp: u32 = 0x10081266;

/// v4.7    KEY_RIGHT_DOWN
pub const FCITX_KEY_RightDown: u32 = 0x10081267;

/// v4.7    KEY_LEFT_UP
pub const FCITX_KEY_LeftUp: u32 = 0x10081268;

/// v4.7    KEY_LEFT_DOWN
pub const FCITX_KEY_LeftDown: u32 = 0x10081269;

/// v4.7    KEY_ROOT_MENU
pub const FCITX_KEY_RootMenu: u32 = 0x1008126a;

/// v4.7    KEY_MEDIA_TOP_MENU
pub const FCITX_KEY_MediaTopMenu: u32 = 0x1008126b;

/// v4.7    KEY_NUMERIC_11
pub const FCITX_KEY_Numeric11: u32 = 0x1008126c;

/// v4.7    KEY_NUMERIC_12
pub const FCITX_KEY_Numeric12: u32 = 0x1008126d;

/// v4.7    KEY_AUDIO_DESC
pub const FCITX_KEY_AudioDesc: u32 = 0x1008126e;

/// v4.7    KEY_3D_MODE
pub const FCITX_KEY_3DMode: u32 = 0x1008126f;

/// v4.7    KEY_NEXT_FAVORITE
pub const FCITX_KEY_NextFavorite: u32 = 0x10081270;

/// v4.7    KEY_STOP_RECORD
pub const FCITX_KEY_StopRecord: u32 = 0x10081271;

/// v4.7    KEY_PAUSE_RECORD
pub const FCITX_KEY_PauseRecord: u32 = 0x10081272;

/// v4.7    KEY_VOD
pub const FCITX_KEY_VOD: u32 = 0x10081273;

/// v4.7    KEY_UNMUTE
pub const FCITX_KEY_Unmute: u32 = 0x10081274;

/// v4.7    KEY_FASTREVERSE
pub const FCITX_KEY_FastReverse: u32 = 0x10081275;

/// v4.7    KEY_SLOWREVERSE
pub const FCITX_KEY_SlowReverse: u32 = 0x10081276;

/// v4.7    KEY_DATA
pub const FCITX_KEY_Data: u32 = 0x10081277;

/// v4.12   KEY_ONSCREEN_KEYBOARD
pub const FCITX_KEY_OnScreenKeyboard: u32 = 0x10081278;

/// v5.5    KEY_PRIVACY_SCREEN_TOGGLE
pub const FCITX_KEY_PrivacyScreenToggle: u32 = 0x10081279;

/// v5.6    KEY_SELECTIVE_SCREENSHOT
pub const FCITX_KEY_SelectiveScreenshot: u32 = 0x1008127a;

/// v5.18   KEY_NEXT_ELEMENT
pub const FCITX_KEY_NextElement: u32 = 0x1008127b;

/// v5.18   KEY_PREVIOUS_ELEMENT
pub const FCITX_KEY_PreviousElement: u32 = 0x1008127c;

/// v5.18   KEY_AUTOPILOT_ENGAGE_TOGGLE
pub const FCITX_KEY_AutopilotEngageToggle: u32 = 0x1008127d;

/// v5.18   KEY_MARK_WAYPOINT
pub const FCITX_KEY_MarkWaypoint: u32 = 0x1008127e;

/// v5.18   KEY_SOS
pub const FCITX_KEY_Sos: u32 = 0x1008127f;

/// v5.18   KEY_NAV_CHART
pub const FCITX_KEY_NavChart: u32 = 0x10081280;

/// v5.18   KEY_FISHING_CHART
pub const FCITX_KEY_FishingChart: u32 = 0x10081281;

/// v5.18   KEY_SINGLE_RANGE_RADAR
pub const FCITX_KEY_SingleRangeRadar: u32 = 0x10081282;

/// v5.18   KEY_DUAL_RANGE_RADAR
pub const FCITX_KEY_DualRangeRadar: u32 = 0x10081283;

/// v5.18   KEY_RADAR_OVERLAY
pub const FCITX_KEY_RadarOverlay: u32 = 0x10081284;

/// v5.18   KEY_TRADITIONAL_SONAR
pub const FCITX_KEY_TraditionalSonar: u32 = 0x10081285;

/// v5.18   KEY_CLEARVU_SONAR
pub const FCITX_KEY_ClearvuSonar: u32 = 0x10081286;

/// v5.18   KEY_SIDEVU_SONAR
pub const FCITX_KEY_SidevuSonar: u32 = 0x10081287;

/// v5.18   KEY_NAV_INFO
pub const FCITX_KEY_NavInfo: u32 = 0x10081288;

/// v5.5    KEY_MACRO1
pub const FCITX_KEY_Macro1: u32 = 0x10081290;

/// v5.5    KEY_MACRO2
pub const FCITX_KEY_Macro2: u32 = 0x10081291;

/// v5.5    KEY_MACRO3
pub const FCITX_KEY_Macro3: u32 = 0x10081292;

/// v5.5    KEY_MACRO4
pub const FCITX_KEY_Macro4: u32 = 0x10081293;

/// v5.5    KEY_MACRO5
pub const FCITX_KEY_Macro5: u32 = 0x10081294;

/// v5.5    KEY_MACRO6
pub const FCITX_KEY_Macro6: u32 = 0x10081295;

/// v5.5    KEY_MACRO7
pub const FCITX_KEY_Macro7: u32 = 0x10081296;

/// v5.5    KEY_MACRO8
pub const FCITX_KEY_Macro8: u32 = 0x10081297;

/// v5.5    KEY_MACRO9
pub const FCITX_KEY_Macro9: u32 = 0x10081298;

/// v5.5    KEY_MACRO10
pub const FCITX_KEY_Macro10: u32 = 0x10081299;

/// v5.5    KEY_MACRO11
pub const FCITX_KEY_Macro11: u32 = 0x1008129a;

/// v5.5    KEY_MACRO12
pub const FCITX_KEY_Macro12: u32 = 0x1008129b;

/// v5.5    KEY_MACRO13
pub const FCITX_KEY_Macro13: u32 = 0x1008129c;

/// v5.5    KEY_MACRO14
pub const FCITX_KEY_Macro14: u32 = 0x1008129d;

/// v5.5    KEY_MACRO15
pub const FCITX_KEY_Macro15: u32 = 0x1008129e;

/// v5.5    KEY_MACRO16
pub const FCITX_KEY_Macro16: u32 = 0x1008129f;

/// v5.5    KEY_MACRO17
pub const FCITX_KEY_Macro17: u32 = 0x100812a0;

/// v5.5    KEY_MACRO18
pub const FCITX_KEY_Macro18: u32 = 0x100812a1;

/// v5.5    KEY_MACRO19
pub const FCITX_KEY_Macro19: u32 = 0x100812a2;

/// v5.5    KEY_MACRO20
pub const FCITX_KEY_Macro20: u32 = 0x100812a3;

/// v5.5    KEY_MACRO21
pub const FCITX_KEY_Macro21: u32 = 0x100812a4;

/// v5.5    KEY_MACRO22
pub const FCITX_KEY_Macro22: u32 = 0x100812a5;

/// v5.5    KEY_MACRO23
pub const FCITX_KEY_Macro23: u32 = 0x100812a6;

/// v5.5    KEY_MACRO24
pub const FCITX_KEY_Macro24: u32 = 0x100812a7;

/// v5.5    KEY_MACRO25
pub const FCITX_KEY_Macro25: u32 = 0x100812a8;

/// v5.5    KEY_MACRO26
pub const FCITX_KEY_Macro26: u32 = 0x100812a9;

/// v5.5    KEY_MACRO27
pub const FCITX_KEY_Macro27: u32 = 0x100812aa;

/// v5.5    KEY_MACRO28
pub const FCITX_KEY_Macro28: u32 = 0x100812ab;

/// v5.5    KEY_MACRO29
pub const FCITX_KEY_Macro29: u32 = 0x100812ac;

/// v5.5    KEY_MACRO30
pub const FCITX_KEY_Macro30: u32 = 0x100812ad;

/// v5.5    KEY_MACRO_RECORD_START
pub const FCITX_KEY_MacroRecordStart: u32 = 0x100812b0;

/// v5.5    KEY_MACRO_RECORD_STOP
pub const FCITX_KEY_MacroRecordStop: u32 = 0x100812b1;

/// v5.5    KEY_MACRO_PRESET_CYCLE
pub const FCITX_KEY_MacroPresetCycle: u32 = 0x100812b2;

/// v5.5    KEY_MACRO_PRESET1
pub const FCITX_KEY_MacroPreset1: u32 = 0x100812b3;

/// v5.5    KEY_MACRO_PRESET2
pub const FCITX_KEY_MacroPreset2: u32 = 0x100812b4;

/// v5.5    KEY_MACRO_PRESET3
pub const FCITX_KEY_MacroPreset3: u32 = 0x100812b5;

/// v5.5    KEY_KBD_LCD_MENU1
pub const FCITX_KEY_KbdLcdMenu1: u32 = 0x100812b8;

/// v5.5    KEY_KBD_LCD_MENU2
pub const FCITX_KEY_KbdLcdMenu2: u32 = 0x100812b9;

/// v5.5    KEY_KBD_LCD_MENU3
pub const FCITX_KEY_KbdLcdMenu3: u32 = 0x100812ba;

/// v5.5    KEY_KBD_LCD_MENU4
pub const FCITX_KEY_KbdLcdMenu4: u32 = 0x100812bb;

/// v5.5    KEY_KBD_LCD_MENU5
pub const FCITX_KEY_KbdLcdMenu5: u32 = 0x100812bc;

/// v6.17   KEY_PERFORMANCE
pub const FCITX_KEY_PerformanceMode: u32 = 0x100812bd;

pub const FCITX_KEY_SunFA_Grave: u32 = 0x1005ff00;

pub const FCITX_KEY_SunFA_Circum: u32 = 0x1005ff01;

pub const FCITX_KEY_SunFA_Tilde: u32 = 0x1005ff02;

pub const FCITX_KEY_SunFA_Acute: u32 = 0x1005ff03;

pub const FCITX_KEY_SunFA_Diaeresis: u32 = 0x1005ff04;

pub const FCITX_KEY_SunFA_Cedilla: u32 = 0x1005ff05;

/// Labeled F11
pub const FCITX_KEY_SunF36: u32 = 0x1005ff10;

/// Labeled F12
pub const FCITX_KEY_SunF37: u32 = 0x1005ff11;

pub const FCITX_KEY_SunSys_Req: u32 = 0x1005ff60;

/// Same as XKB_KEY_Print
pub const FCITX_KEY_SunPrint_Screen: u32 = 0x0000ff61;

/// Same as XKB_KEY_Multi_key
pub const FCITX_KEY_SunCompose: u32 = 0x0000ff20;

/// Same as XKB_KEY_Mode_switch
pub const FCITX_KEY_SunAltGraph: u32 = 0x0000ff7e;

/// Same as XKB_KEY_Prior
pub const FCITX_KEY_SunPageUp: u32 = 0x0000ff55;

/// Same as XKB_KEY_Next
pub const FCITX_KEY_SunPageDown: u32 = 0x0000ff56;

/// Same as XKB_KEY_Undo
pub const FCITX_KEY_SunUndo: u32 = 0x0000ff65;

/// Same as XKB_KEY_Redo
pub const FCITX_KEY_SunAgain: u32 = 0x0000ff66;

/// Same as XKB_KEY_Find
pub const FCITX_KEY_SunFind: u32 = 0x0000ff68;

/// Same as XKB_KEY_Cancel
pub const FCITX_KEY_SunStop: u32 = 0x0000ff69;

pub const FCITX_KEY_SunProps: u32 = 0x1005ff70;

pub const FCITX_KEY_SunFront: u32 = 0x1005ff71;

pub const FCITX_KEY_SunCopy: u32 = 0x1005ff72;

pub const FCITX_KEY_SunOpen: u32 = 0x1005ff73;

pub const FCITX_KEY_SunPaste: u32 = 0x1005ff74;

pub const FCITX_KEY_SunCut: u32 = 0x1005ff75;

pub const FCITX_KEY_SunPowerSwitch: u32 = 0x1005ff76;

pub const FCITX_KEY_SunAudioLowerVolume: u32 = 0x1005ff77;

pub const FCITX_KEY_SunAudioMute: u32 = 0x1005ff78;

pub const FCITX_KEY_SunAudioRaiseVolume: u32 = 0x1005ff79;

pub const FCITX_KEY_SunVideoDegauss: u32 = 0x1005ff7a;

pub const FCITX_KEY_SunVideoLowerBrightness: u32 = 0x1005ff7b;

pub const FCITX_KEY_SunVideoRaiseBrightness: u32 = 0x1005ff7c;

pub const FCITX_KEY_SunPowerSwitchShift: u32 = 0x1005ff7d;

pub const FCITX_KEY_Dring_accent: u32 = 0x1000feb0;

pub const FCITX_KEY_Dcircumflex_accent: u32 = 0x1000fe5e;

pub const FCITX_KEY_Dcedilla_accent: u32 = 0x1000fe2c;

pub const FCITX_KEY_Dacute_accent: u32 = 0x1000fe27;

pub const FCITX_KEY_Dgrave_accent: u32 = 0x1000fe60;

pub const FCITX_KEY_Dtilde: u32 = 0x1000fe7e;

pub const FCITX_KEY_Ddiaeresis: u32 = 0x1000fe22;

/// Remove
pub const FCITX_KEY_DRemove: u32 = 0x1000ff00;

pub const FCITX_KEY_hpClearLine: u32 = 0x1000ff6f;

pub const FCITX_KEY_hpInsertLine: u32 = 0x1000ff70;

pub const FCITX_KEY_hpDeleteLine: u32 = 0x1000ff71;

pub const FCITX_KEY_hpInsertChar: u32 = 0x1000ff72;

pub const FCITX_KEY_hpDeleteChar: u32 = 0x1000ff73;

pub const FCITX_KEY_hpBackTab: u32 = 0x1000ff74;

pub const FCITX_KEY_hpKP_BackTab: u32 = 0x1000ff75;

pub const FCITX_KEY_hpModelock1: u32 = 0x1000ff48;

pub const FCITX_KEY_hpModelock2: u32 = 0x1000ff49;

pub const FCITX_KEY_hpReset: u32 = 0x1000ff6c;

pub const FCITX_KEY_hpSystem: u32 = 0x1000ff6d;

pub const FCITX_KEY_hpUser: u32 = 0x1000ff6e;

pub const FCITX_KEY_hpmute_acute: u32 = 0x100000a8;

pub const FCITX_KEY_hpmute_grave: u32 = 0x100000a9;

pub const FCITX_KEY_hpmute_asciicircum: u32 = 0x100000aa;

pub const FCITX_KEY_hpmute_diaeresis: u32 = 0x100000ab;

pub const FCITX_KEY_hpmute_asciitilde: u32 = 0x100000ac;

pub const FCITX_KEY_hplira: u32 = 0x100000af;

pub const FCITX_KEY_hpguilder: u32 = 0x100000be;

pub const FCITX_KEY_hpYdiaeresis: u32 = 0x100000ee;

/// deprecated alias for hpYdiaeresis
pub const FCITX_KEY_hpIO: u32 = 0x100000ee;

pub const FCITX_KEY_hplongminus: u32 = 0x100000f6;

pub const FCITX_KEY_hpblock: u32 = 0x100000fc;

pub const FCITX_KEY_osfCopy: u32 = 0x1004ff02;

pub const FCITX_KEY_osfCut: u32 = 0x1004ff03;

pub const FCITX_KEY_osfPaste: u32 = 0x1004ff04;

pub const FCITX_KEY_osfBackTab: u32 = 0x1004ff07;

pub const FCITX_KEY_osfBackSpace: u32 = 0x1004ff08;

pub const FCITX_KEY_osfClear: u32 = 0x1004ff0b;

pub const FCITX_KEY_osfEscape: u32 = 0x1004ff1b;

pub const FCITX_KEY_osfAddMode: u32 = 0x1004ff31;

pub const FCITX_KEY_osfPrimaryPaste: u32 = 0x1004ff32;

pub const FCITX_KEY_osfQuickPaste: u32 = 0x1004ff33;

pub const FCITX_KEY_osfPageLeft: u32 = 0x1004ff40;

pub const FCITX_KEY_osfPageUp: u32 = 0x1004ff41;

pub const FCITX_KEY_osfPageDown: u32 = 0x1004ff42;

pub const FCITX_KEY_osfPageRight: u32 = 0x1004ff43;

pub const FCITX_KEY_osfActivate: u32 = 0x1004ff44;

pub const FCITX_KEY_osfMenuBar: u32 = 0x1004ff45;

pub const FCITX_KEY_osfLeft: u32 = 0x1004ff51;

pub const FCITX_KEY_osfUp: u32 = 0x1004ff52;

pub const FCITX_KEY_osfRight: u32 = 0x1004ff53;

pub const FCITX_KEY_osfDown: u32 = 0x1004ff54;

pub const FCITX_KEY_osfEndLine: u32 = 0x1004ff57;

pub const FCITX_KEY_osfBeginLine: u32 = 0x1004ff58;

pub const FCITX_KEY_osfEndData: u32 = 0x1004ff59;

pub const FCITX_KEY_osfBeginData: u32 = 0x1004ff5a;

pub const FCITX_KEY_osfPrevMenu: u32 = 0x1004ff5b;

pub const FCITX_KEY_osfNextMenu: u32 = 0x1004ff5c;

pub const FCITX_KEY_osfPrevField: u32 = 0x1004ff5d;

pub const FCITX_KEY_osfNextField: u32 = 0x1004ff5e;

pub const FCITX_KEY_osfSelect: u32 = 0x1004ff60;

pub const FCITX_KEY_osfInsert: u32 = 0x1004ff63;

pub const FCITX_KEY_osfUndo: u32 = 0x1004ff65;

pub const FCITX_KEY_osfMenu: u32 = 0x1004ff67;

pub const FCITX_KEY_osfCancel: u32 = 0x1004ff69;

pub const FCITX_KEY_osfHelp: u32 = 0x1004ff6a;

pub const FCITX_KEY_osfSelectAll: u32 = 0x1004ff71;

pub const FCITX_KEY_osfDeselectAll: u32 = 0x1004ff72;

pub const FCITX_KEY_osfReselect: u32 = 0x1004ff73;

pub const FCITX_KEY_osfExtend: u32 = 0x1004ff74;

pub const FCITX_KEY_osfRestore: u32 = 0x1004ff78;

pub const FCITX_KEY_osfDelete: u32 = 0x1004ffff;

/// deprecated alias for hpReset
pub const FCITX_KEY_Reset: u32 = 0x1000ff6c;

/// deprecated alias for hpSystem
pub const FCITX_KEY_System: u32 = 0x1000ff6d;

/// deprecated alias for hpUser
pub const FCITX_KEY_User: u32 = 0x1000ff6e;

/// deprecated alias for hpClearLine
pub const FCITX_KEY_ClearLine: u32 = 0x1000ff6f;

/// deprecated alias for hpInsertLine
pub const FCITX_KEY_InsertLine: u32 = 0x1000ff70;

/// deprecated alias for hpDeleteLine
pub const FCITX_KEY_DeleteLine: u32 = 0x1000ff71;

/// deprecated alias for hpInsertChar
pub const FCITX_KEY_InsertChar: u32 = 0x1000ff72;

/// deprecated alias for hpDeleteChar
pub const FCITX_KEY_DeleteChar: u32 = 0x1000ff73;

/// deprecated alias for hpBackTab
pub const FCITX_KEY_BackTab: u32 = 0x1000ff74;

/// deprecated alias for hpKP_BackTab
pub const FCITX_KEY_KP_BackTab: u32 = 0x1000ff75;

/// deprecated
pub const FCITX_KEY_Ext16bit_L: u32 = 0x1000ff76;

/// deprecated
pub const FCITX_KEY_Ext16bit_R: u32 = 0x1000ff77;

/// deprecated alias for hpmute_acute
pub const FCITX_KEY_mute_acute: u32 = 0x100000a8;

/// deprecated alias for hpmute_grave
pub const FCITX_KEY_mute_grave: u32 = 0x100000a9;

/// deprecated alias for hpmute_asciicircum
pub const FCITX_KEY_mute_asciicircum: u32 = 0x100000aa;

/// deprecated alias for hpmute_diaeresis
pub const FCITX_KEY_mute_diaeresis: u32 = 0x100000ab;

/// deprecated alias for hpmute_asciitilde
pub const FCITX_KEY_mute_asciitilde: u32 = 0x100000ac;

/// deprecated alias for hplira
pub const FCITX_KEY_lira: u32 = 0x100000af;

/// deprecated alias for hpguilder
pub const FCITX_KEY_guilder: u32 = 0x100000be;

/// deprecated alias for hpYdiaeresis
pub const FCITX_KEY_IO: u32 = 0x100000ee;

/// deprecated alias for hplongminus
pub const FCITX_KEY_longminus: u32 = 0x100000f6;

/// deprecated alias for hpblock
pub const FCITX_KEY_block: u32 = 0x100000fc;
