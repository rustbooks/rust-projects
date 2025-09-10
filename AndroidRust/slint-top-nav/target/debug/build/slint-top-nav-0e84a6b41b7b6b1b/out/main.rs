mod slint_generatedAppWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#MenuItem {
         pub r#icon : sp :: SharedString , pub r#name : sp :: SharedString }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_13_0 = slint :: VersionCheck_1_13_0 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerAppLogic {
         r#current_menu : sp :: Property < i32 > , r#menu_items : sp :: Property < sp :: ModelRc < r#MenuItem > > , r#popup_message : sp :: Property < sp :: SharedString > , r#popup_visible : sp :: Property < bool > , r#close_popup : sp :: Callback < () , () > , r#menu_selected : sp :: Callback < (i32 ,) , () > , r#submit : sp :: Callback < (sp :: SharedString , sp :: SharedString , sp :: SharedString ,) , () > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerAppLogic {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 * & InnerAppLogic :: FIELD_OFFSETS . r#current_menu }
             . apply_pin (_self) . set ({
                 (((0f64) as i32)) as i32 }
            ) ;
             {
                 * & InnerAppLogic :: FIELD_OFFSETS . r#popup_visible }
             . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             }
         }
     # [allow (unused)] pub struct r#AppLogic < 'a > (& 'a :: core :: pin :: Pin < sp :: Rc < InnerAppLogic >>) ;
     impl < 'a > r#AppLogic < 'a > {
         # [allow (dead_code)] pub fn invoke_close_popup (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#close_popup . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . call (& ()) }
         # [allow (dead_code)] pub fn on_close_popup (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] InnerAppLogic :: FIELD_OFFSETS . r#close_popup . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_current_menu (& self) -> i32 {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#current_menu . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_current_menu (& self , value : i32) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#current_menu . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn get_menu_items (& self) -> sp :: ModelRc < r#MenuItem > {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#menu_items . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_menu_items (& self , value : sp :: ModelRc < r#MenuItem >) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#menu_items . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_menu_selected (& self , arg_0 : i32 ,) -> () {
             let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#menu_selected . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_menu_selected (& self , mut f : impl FnMut (i32) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] InnerAppLogic :: FIELD_OFFSETS . r#menu_selected . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_popup_message (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#popup_message . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_popup_message (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#popup_message . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn get_popup_visible (& self) -> bool {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#popup_visible . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_popup_visible (& self , value : bool) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#popup_visible . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_submit (& self , arg_0 : sp :: SharedString , arg_1 : sp :: SharedString , arg_2 : sp :: SharedString ,) -> () {
             let _self = self . 0 . as_ref () ;
             InnerAppLogic :: FIELD_OFFSETS . r#submit . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . call (& (arg_0 , arg_1 , arg_2 ,)) }
         # [allow (dead_code)] pub fn on_submit (& self , mut f : impl FnMut (sp :: SharedString , sp :: SharedString , sp :: SharedString) -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] InnerAppLogic :: FIELD_OFFSETS . r#submit . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_AppLogic . as_ref ()) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone () , args . 2 . clone ())) }
         }
     impl < 'a > slint :: Global < 'a , r#AppWindow > for r#AppLogic < 'a > {
         fn get (component : & 'a r#AppWindow) -> Self {
             Self (& component . 0 . globals . get () . unwrap () . global_AppLogic) }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStrings {
         r#app_name : sp :: Property < sp :: SharedString > , r#submit_button : sp :: Property < sp :: SharedString > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerStrings {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 * & InnerStrings :: FIELD_OFFSETS . r#app_name }
             . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Top Nav")) as sp :: SharedString }
            ) ;
             {
                 * & InnerStrings :: FIELD_OFFSETS . r#submit_button }
             . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Submit")) as sp :: SharedString }
            ) ;
             }
         }
     # [allow (unused)] pub struct r#Strings < 'a > (& 'a :: core :: pin :: Pin < sp :: Rc < InnerStrings >>) ;
     impl < 'a > r#Strings < 'a > {
         # [allow (dead_code)] pub fn get_app_name (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerStrings :: FIELD_OFFSETS . r#app_name . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Strings . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_app_name (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerStrings :: FIELD_OFFSETS . r#app_name . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Strings . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn get_submit_button (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerStrings :: FIELD_OFFSETS . r#submit_button . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Strings . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_submit_button (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerStrings :: FIELD_OFFSETS . r#submit_button . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Strings . as_ref ()) . set (value as _) }
         }
     impl < 'a > slint :: Global < 'a , r#AppWindow > for r#Strings < 'a > {
         fn get (component : & 'a r#AppWindow) -> Self {
             Self (& component . 0 . globals . get () . unwrap () . global_Strings) }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerTheme {
         r#accent_color : sp :: Property < sp :: Color > , r#background_color : sp :: Property < sp :: Color > , r#primary_color : sp :: Property < sp :: Color > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerTheme {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 * & InnerTheme :: FIELD_OFFSETS . r#accent_color }
             . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((4278442694f64) as u32)) as sp :: Color }
            ) ;
             {
                 * & InnerTheme :: FIELD_OFFSETS . r#background_color }
             . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((4279374354f64) as u32)) as sp :: Color }
            ) ;
             {
                 * & InnerTheme :: FIELD_OFFSETS . r#primary_color }
             . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((4284612846f64) as u32)) as sp :: Color }
            ) ;
             }
         }
     # [allow (unused)] pub struct r#Theme < 'a > (& 'a :: core :: pin :: Pin < sp :: Rc < InnerTheme >>) ;
     impl < 'a > r#Theme < 'a > {
         # [allow (dead_code)] pub fn get_accent_color (& self) -> sp :: Color {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerTheme :: FIELD_OFFSETS . r#accent_color . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Theme . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_accent_color (& self , value : sp :: Color) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerTheme :: FIELD_OFFSETS . r#accent_color . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Theme . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn get_background_color (& self) -> sp :: Color {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerTheme :: FIELD_OFFSETS . r#background_color . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Theme . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_background_color (& self , value : sp :: Color) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerTheme :: FIELD_OFFSETS . r#background_color . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Theme . as_ref ()) . set (value as _) }
         # [allow (dead_code)] pub fn get_primary_color (& self) -> sp :: Color {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerTheme :: FIELD_OFFSETS . r#primary_color . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Theme . as_ref ()) . get () }
         # [allow (dead_code)] pub fn set_primary_color (& self , value : sp :: Color) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             InnerTheme :: FIELD_OFFSETS . r#primary_color . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_Theme . as_ref ()) . set (value as _) }
         }
     impl < 'a > slint :: Global < 'a , r#AppWindow > for r#Theme < 'a > {
         fn get (component : & 'a r#AppWindow) -> Self {
             Self (& component . 0 . globals . get () . unwrap () . global_Theme) }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_112 {
         r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerFluentPalette_112 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_112 :: FIELD_OFFSETS . r#color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_FluentPalette_112_color_scheme = InnerFluentPalette_112 :: FIELD_OFFSETS . r#color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () ;
                         if ! (((r#tmp_FluentPalette_112_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Unknown))) {
                             ((((r#tmp_FluentPalette_112_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Dark)))) as _ }
                         else {
                             ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ())) == ((sp :: r#ColorScheme :: r#Dark)) }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_112 :: FIELD_OFFSETS . r#foreground }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerTextEdit_root_1 {
         r#root_1 : sp :: r#Empty , r#base_2 : sp :: r#BasicBorderRectangle , r#contextmenuinternal_3 : sp :: r#ContextMenu , r#scroll_view_4 : sp :: r#Empty , r#flickable_5 : sp :: r#Flickable , r#flickable_viewport_6 : sp :: r#Empty , r#text_input_7 : sp :: r#TextInput , r#vertical_bar_visibility_8 : sp :: r#Clip , r#vertical_bar_9 : sp :: r#BasicBorderRectangle , r#vertical_bar_clip_10 : sp :: r#Clip , r#thumb_11 : sp :: r#BasicBorderRectangle , r#touch_area_12 : sp :: r#TouchArea , r#up_scroll_button_opacity_13 : sp :: r#Opacity , r#up_scroll_button_14 : sp :: r#TouchArea , r#icon_opacity_15 : sp :: r#Opacity , r#icon_16 : sp :: r#ImageItem , r#down_scroll_button_opacity_17 : sp :: r#Opacity , r#down_scroll_button_18 : sp :: r#TouchArea , r#icon_opacity_19 : sp :: r#Opacity , r#icon_20 : sp :: r#ImageItem , r#horizontal_bar_visibility_21 : sp :: r#Clip , r#horizontal_bar_22 : sp :: r#BasicBorderRectangle , r#horizontal_bar_clip_23 : sp :: r#Clip , r#thumb_24 : sp :: r#BasicBorderRectangle , r#touch_area_25 : sp :: r#TouchArea , r#up_scroll_button_opacity_26 : sp :: r#Opacity , r#up_scroll_button_27 : sp :: r#TouchArea , r#icon_opacity_28 : sp :: r#Opacity , r#icon_29 : sp :: r#ImageItem , r#down_scroll_button_opacity_30 : sp :: r#Opacity , r#down_scroll_button_31 : sp :: r#TouchArea , r#icon_opacity_32 : sp :: r#Opacity , r#icon_33 : sp :: r#ImageItem , r#placeholder_34 : sp :: r#ComplexText , r#i_focus_border_35 : sp :: r#Rectangle , r#root_1_accessible_placeholder_text : sp :: Property < sp :: SharedString > , r#root_1_base_2_height : sp :: Property < sp :: LogicalLength > , r#root_1_base_2_placeholder_text : sp :: Property < sp :: SharedString > , r#root_1_base_2_selection_foreground_color : sp :: Property < slint :: Brush > , r#root_1_base_2_visible_height : sp :: Property < sp :: LogicalLength > , r#root_1_base_2_visible_width : sp :: Property < sp :: LogicalLength > , r#root_1_base_2_width : sp :: Property < sp :: LogicalLength > , r#root_1_down_scroll_button_18_state : sp :: Property < i32 > , r#root_1_down_scroll_button_31_state : sp :: Property < i32 > , r#root_1_flickable_5_horizontal_stretch : sp :: Property < f32 > , r#root_1_flickable_5_max_height : sp :: Property < sp :: LogicalLength > , r#root_1_flickable_5_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_flickable_5_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_flickable_5_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_flickable_5_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_flickable_5_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_flickable_5_vertical_stretch : sp :: Property < f32 > , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_horizontal_bar_22_maximum : sp :: Property < sp :: LogicalLength > , r#root_1_horizontal_bar_22_policy : sp :: Property < sp :: r#ScrollBarPolicy > , r#root_1_horizontal_bar_22_size : sp :: Property < sp :: LogicalLength > , r#root_1_horizontal_bar_22_state : sp :: Property < i32 > , r#root_1_horizontal_bar_22_visible : sp :: Property < bool > , r#root_1_placeholder_34_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_placeholder_34_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_scroll_view_4_vertical_scrollbar_policy : sp :: Property < sp :: r#ScrollBarPolicy > , r#root_1_state : sp :: Property < i32 > , r#root_1_text_input_7_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_input_7_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_thumb_11_height : sp :: Property < sp :: LogicalLength > , r#root_1_thumb_11_width : sp :: Property < sp :: LogicalLength > , r#root_1_thumb_11_y : sp :: Property < sp :: LogicalLength > , r#root_1_thumb_24_height : sp :: Property < sp :: LogicalLength > , r#root_1_thumb_24_width : sp :: Property < sp :: LogicalLength > , r#root_1_thumb_24_x : sp :: Property < sp :: LogicalLength > , r#root_1_touch_area_12_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_1_touch_area_25_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_1_up_scroll_button_14_state : sp :: Property < i32 > , r#root_1_up_scroll_button_27_state : sp :: Property < i32 > , r#root_1_vertical_bar_9_maximum : sp :: Property < sp :: LogicalLength > , r#root_1_vertical_bar_9_size : sp :: Property < sp :: LogicalLength > , r#root_1_vertical_bar_9_state : sp :: Property < i32 > , r#root_1_vertical_bar_9_visible : sp :: Property < bool > , r#root_1_width : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_base_2_edited : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_base_2_key_pressed : sp :: Callback < (sp :: KeyEvent ,) , sp :: r#EventResult > , r#root_1_base_2_key_released : sp :: Callback < (sp :: KeyEvent ,) , sp :: r#EventResult > , r#root_1_horizontal_bar_22_scrolled : sp :: Callback < () , () > , r#root_1_vertical_bar_9_scrolled : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerTextEdit_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerTextEdit_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_accessible_placeholder_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ())) == ((sp :: SharedString :: from (""))) {
                         (({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_placeholder_text }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1f64) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_selection_foreground_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((24f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((24f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1f64) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_down_scroll_button_18_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_down_scroll_button_31_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_flickable_5_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 6u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_policy }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#ScrollBarPolicy :: r#AsNeeded) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_scrolled }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#flicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) || ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())))) || ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_1_horizontal_bar_22_policy = ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_policy }
                        ) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_1_horizontal_bar_22_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AlwaysOn)))) || ((((((r#tmp_root_1_horizontal_bar_22_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AsNeeded)))) && ((((({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                        ) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_34_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_34_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_scroll_view_4_vertical_scrollbar_policy }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#ScrollBarPolicy :: r#AsNeeded) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                         + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_text_input_7_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_text_input_7_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_1_vertical_bar_9_maximum = ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ((if ((r#tmp_root_1_vertical_bar_9_maximum . clone ()) as f64) <= ((((0f64) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64)) as f64) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_1_vertical_bar_9_page_size = ({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                ) . apply_pin (_self) . get () . get () ;
                                 ((((16f64 as sp :: Coord) . min ((((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord) as sp :: Coord) . max ((((((((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) * ((((r#tmp_root_1_vertical_bar_9_page_size . clone ()) as f64) / ((((r#tmp_root_1_vertical_bar_9_maximum . clone ()) as f64) + ((r#tmp_root_1_vertical_bar_9_page_size . clone ()) as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                             }
                        ) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_size }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((16f64) as f64) + ((((((((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((- ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) / ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_size }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_1_horizontal_bar_22_maximum = ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ((if ((r#tmp_root_1_horizontal_bar_22_maximum . clone ()) as f64) <= ((((0f64) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64)) as f64) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_1_horizontal_bar_22_page_size = ({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                ) . apply_pin (_self) . get () . get () ;
                                 ((((16f64 as sp :: Coord) . min ((((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord) as sp :: Coord) . max ((((((((((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) * ((r#tmp_root_1_horizontal_bar_22_page_size . clone ()) as f64)) as f64) / ((((r#tmp_root_1_horizontal_bar_22_maximum . clone ()) as f64) + ((r#tmp_root_1_horizontal_bar_22_page_size . clone ()) as f64)) as f64)) as sp :: Coord)) as f64) * ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                             }
                        ) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((16f64) as f64) + ((((((((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((- ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) / ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_up_scroll_button_14_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_up_scroll_button_27_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_scrolled }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#flicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) || ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())))) || ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_1_scroll_view_4_vertical_scrollbar_policy = ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_scroll_view_4_vertical_scrollbar_policy }
                        ) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_1_scroll_view_4_vertical_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AlwaysOn)))) || ((((((r#tmp_root_1_scroll_view_4_vertical_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AsNeeded)))) && ((((({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                        ) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_1_state = ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (1f64 as f64)) {
                             (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((3005095454f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_1_state = ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (1f64 as f64)) {
                             (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                         color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9998f64 as _ }
                                     , sp :: GradientStop {
                                         color : sp :: Color :: from_argb_encoded ((2332033023f64) as u32) , position : 1f64 as _ }
                                     , sp :: GradientStop {
                                         color : sp :: Color :: from_argb_encoded ((2332033023f64) as u32) , position : 1f64 as _ }
                                    ]))) as _ }
                                 else {
                                     slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                         color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9999f64 as _ }
                                     , sp :: GradientStop {
                                         color : sp :: Color :: from_argb_encoded ((1929379840f64) as u32) , position : 1f64 as _ }
                                     , sp :: GradientStop {
                                         color : sp :: Color :: from_argb_encoded ((1929379840f64) as u32) , position : 1f64 as _ }
                                    ])) }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_98 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let window_adapter = & _self . globals . get () . unwrap () . window_adapter_impl () ;
                             let menu_item_tree_instance = InnerComponent_empty_36 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () ;
                             let context_menu_item_tree = sp :: VRc :: new (sp :: MenuFromItemTree :: new (sp :: VRc :: into_dyn (menu_item_tree_instance))) ;
                             let context_menu_item_tree_ = context_menu_item_tree . clone () ;
                             {
                                 let mut entries = sp :: SharedVector :: default () ;
                                 sp :: Menu :: sub_menu (& * context_menu_item_tree , sp :: Option :: None , & mut entries) ;
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                ) . apply_pin (_self) . set (sp :: ModelRc :: new (sp :: SharedVectorModel :: from (entries))) ;
                                 let context_menu_item_tree = context_menu_item_tree_ . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     let mut entries = sp :: SharedVector :: default () ;
                                     sp :: Menu :: sub_menu (& * context_menu_item_tree , sp :: Option :: Some (& entry . 0) , & mut entries) ;
                                     sp :: ModelRc :: new (sp :: SharedVectorModel :: from (entries)) }
                                ) ;
                                 let context_menu_item_tree = context_menu_item_tree_ . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_activated }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     sp :: Menu :: activate (& * context_menu_item_tree_ , & entry . 0) ;
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_close }
                                ) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
                                    ) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             let context_menu_item_tree = sp :: VRc :: into_dyn (context_menu_item_tree) ;
                             if ! sp :: WindowInner :: from_pub (window_adapter . window ()) . show_native_popup_menu (context_menu_item_tree , position , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) {
                                 if let Some (current_id) = ({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
                                ) . apply_pin (_self) . popup_id . take () {
                                     sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                     }
                                 let id = sp :: WindowInner :: from_pub (window_adapter . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true ,) ;
                                 ({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
                                ) . apply_pin (_self) . popup_id . set (Some (id)) ;
                                 InnerPopupMenuImpl_root_98 :: user_init (popup_instance_vrc) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_text_input_7_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . get ())) == ((sp :: r#TextWrap :: r#NoWrap)) {
                         ((({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_text_input_7_preferred_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                     else {
                         ({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                        ) . apply_pin (_self) . get () . get () }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x) as f64) + ((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64)) as f64) < ((12f64) as f64) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                    ) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord) . max ((((- (args . 0 . clone ()) . r#x) as f64) + ((12f64) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#x) as f64) + ((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64)) as f64) > ((((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((12f64) as f64)) as f64) {
                                     ({
                                         ({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord) . max ((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - (((args . 0 . clone ()) . r#x) as f64)) as f64) - ((12f64) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             ;
                             if (((((args . 0 . clone ()) . r#y) as f64) + ((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64)) as f64) < ((12f64) as f64) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                    ) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord) . max ((((- (args . 0 . clone ()) . r#y) as f64) + ((12f64) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#y) as f64) + ((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64)) as f64) > ((((((({
                                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((12f64) as f64)) as f64) - ((20f64) as f64)) as f64) {
                                     ({
                                         ({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord) . max ((((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - (((args . 0 . clone ()) . r#y) as f64)) as f64) - ((12f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1.0766f64) as f64) * ((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((400f64) as i32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_key_pressed }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#key_released) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_key_released }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#page_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4278221012f64) as u32)) . color ()) as sp :: Color }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_selection_foreground_color }
                    ) . apply_pin (_self) . get () . color ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#TextWrap :: r#WordWrap) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4281084972f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293980400f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_11 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_11 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true)) && ((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max ((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_touch_area_12_pressed_value }
                                    ) . apply_pin (_self) . get () . get ()) as f64) + ((if false {
                                         ((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64) }
                                    ) as f64)) as sp :: Coord) as sp :: Coord)) as sp :: Coord) as _) ;
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_scrolled }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_touch_area_12_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ((false)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_x) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if ! (((! false)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ((true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_y) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge0 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression0 . clone ()) . 1 {
                                 ((r#returned_expression0 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression0 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_opacity_13 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_15 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_up_scroll_button_14_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_up_scroll_button_14_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         8f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_opacity_17 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_vertical_bar_9_maximum }
                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_19 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_down_scroll_button_18_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_down_scroll_button_18_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         8f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4281084972f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293980400f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_24 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_24 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true)) && ((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max ((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_touch_area_25_pressed_value }
                                    ) . apply_pin (_self) . get () . get ()) as f64) + ((if true {
                                         ((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64) }
                                    ) as f64)) as sp :: Coord) as sp :: Coord)) as sp :: Coord) as _) ;
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_scrolled }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_touch_area_25_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((true)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_x) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if ! (((! true)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ((true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_y) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 ((r#returned_expression1 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression1 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_opacity_26 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_28 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_up_scroll_button_27_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_up_scroll_button_27_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         6f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_opacity_30 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_horizontal_bar_22_maximum }
                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((((({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_32 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_down_scroll_button_31_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_down_scroll_button_31_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         6f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_34_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_placeholder_34_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set ({
                 (sp :: r#TextOverflow :: r#Elide) as sp :: r#TextOverflow }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ())) == ((sp :: SharedString :: from (""))))) && ((((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ())) == ((sp :: SharedString :: from (""))))) {
                         (({
                             * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_placeholder_text }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Top) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((1f64) as f64) * ((({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) - ((24f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#i_focus_border_35 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4284534271f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4278214584f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             {
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 ;
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let r#layout_info_0 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_0 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_0 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (r#layout_info_0 . clone ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let r#layout_info_1 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (r#layout_info_1 . clone ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => ((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , (((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as f64) - ((24f64) as f64)) as sp :: Coord , 12f64 as sp :: Coord , 12f64 as sp :: Coord ,) , 4u32 => (2f64 as sp :: Coord , (((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , 4f64 as sp :: Coord , (((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as f64) - ((2f64) as f64)) as sp :: Coord ,) , 5u32 => ((((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as f64) - ((24f64) as f64)) as sp :: Coord , (((((1f64) as f64) * ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as f64) - ((24f64) as f64)) as sp :: Coord , 12f64 as sp :: Coord , 12f64 as sp :: Coord ,) , 6u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 7u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 8u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 9u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 10u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 11u32 => ((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 14f64 as sp :: Coord , (((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) . get () . get ()) as f64) + ((0f64) as f64)) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord ,) , 12u32 => ((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 14f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 13u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((10f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_11_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 14u32 => ((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 14f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 15u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 3f64 as sp :: Coord , 4f64 as sp :: Coord ,) , 16u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 3f64 as sp :: Coord , (((((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((6f64) as f64)) as f64) - ((4f64) as f64)) as sp :: Coord ,) , 17u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 18u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 19u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 20u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 21u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 22u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 23u32 => (14f64 as sp :: Coord , (((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord , (((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_height }
                ) . apply_pin (_self) . get () . get ()) as f64) + ((0f64) as f64)) as f64) - ((14f64) as f64)) as sp :: Coord ,) , 24u32 => (14f64 as sp :: Coord , (((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 25u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((10f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_thumb_24_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord ,) , 26u32 => (14f64 as sp :: Coord , (((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 27u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 4f64 as sp :: Coord , 3f64 as sp :: Coord ,) , 28u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , (((((((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_base_2_visible_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((6f64) as f64)) as f64) - ((4f64) as f64)) as sp :: Coord , 3f64 as sp :: Coord ,) , 29u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 30u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 31u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 32u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 33u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 34u32 => (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#TextInput , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (0u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1) , false , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) . apply_pin (_self) . r#clear_selection (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) . apply_pin (_self) . r#copy (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) . apply_pin (_self) . r#cut (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_paste (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) . apply_pin (_self) . r#paste (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) . apply_pin (_self) . r#select_all (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_base_2_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             (({
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) . apply_pin (_self) . set_selection_offsets (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_empty_36 {
         r#empty_36 : sp :: r#Empty , r#menuitem_37 : sp :: r#MenuItem , r#menuitem_38 : sp :: r#MenuItem , r#menuitem_39 : sp :: r#MenuItem , r#menuitem_40 : sp :: r#MenuItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_empty_36 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTextEdit_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_empty_36 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                            ) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#cut (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 10u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((! _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())) && ((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Cut")) as _ , (sp :: SharedString :: from ("TextEditBase")) as _ , (sp :: SharedString :: from ("slint-top-nav")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                            ) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#copy (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 10u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . is_empty ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Copy")) as _ , (sp :: SharedString :: from ("TextEditBase")) as _ , (sp :: SharedString :: from ("slint-top-nav")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                            ) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#paste (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 10u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((! _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())) && ((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Paste")) as _ , (sp :: SharedString :: from ("TextEditBase")) as _ , (sp :: SharedString :: from ("slint-top-nav")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             _self . parent . upgrade () . as_ref () . map (| x | ({
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                            ) . apply_pin (x . as_pin_ref ())) . as_ref () . map (| x | x . r#select_all (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 10u32 - 1))) . unwrap_or_default () }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checked) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . is_empty ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
                 + sp :: r#MenuItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Select All")) as _ , (sp :: SharedString :: from ("TextEditBase")) as _ , (sp :: SharedString :: from ("slint-top-nav")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#checkable) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
             + sp :: r#MenuItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#empty_36 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#empty_36 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_empty_36 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTextEdit_root_1 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTextEdit_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 4u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_empty_36 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#empty_36 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_37 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_38 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_39 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_36 :: FIELD_OFFSETS . r#menuitem_40 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_empty_36) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_empty_36 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_empty_36 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_empty_36 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_empty_36 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some (parent_rc) = self . parent . clone () . upgrade () {
                 let parent_origin = sp :: VRcMapped :: origin (& parent_rc) ;
                 * _result = sp :: ItemRc :: new (parent_origin , 0) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFocusBorder_root_41 {
         r#root_41 : sp :: r#BasicBorderRectangle , r#rectangle_42 : sp :: r#BasicBorderRectangle , r#root_41_height : sp :: Property < sp :: LogicalLength > , r#root_41_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_41 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_41 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#rectangle_42 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3003121664f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#rectangle_42 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
                     + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . get () . get ()) as f64) - ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#rectangle_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#rectangle_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#rectangle_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((((({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((4f64) as f64)) as sp :: Coord , (((({
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((4f64) as f64)) as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_43 {
         r#root_43 : sp :: r#Empty , r#i_background_44 : sp :: r#BasicBorderRectangle , r#i_border_45 : sp :: r#BasicBorderRectangle , r#i_touch_area_51 : sp :: r#TouchArea , r#i_focus_scope_52 : sp :: r#FocusScope , r#root_43_checked : sp :: Property < bool > , r#root_43_has_focus : sp :: Property < bool > , r#root_43_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_background_44_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_layout_46_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_43_i_layout_46_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_43_i_layout_46_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_43_icon : sp :: Property < sp :: Image > , r#root_43_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_43_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_43_pressed : sp :: Property < bool > , r#root_43_state : sp :: Property < i32 > , r#root_43_text : sp :: Property < sp :: SharedString > , r#root_43_text_color : sp :: Property < slint :: Brush > , r#root_43_width : sp :: Property < sp :: LogicalLength > , r#root_43_x : sp :: Property < sp :: LogicalLength > , r#root_43_y : sp :: Property < sp :: LogicalLength > , r#root_43_accessible_action_default : sp :: Callback < () , () > , r#root_43_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_image_47 > , repeater1 : sp :: Conditional < InnerComponent_text_49 > , repeater2 : sp :: Conditional < InnerComponent_focusborder_53 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_43 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_43 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((((sp :: Image :: default () . size ()) . r#width) as f64) > ((0f64) as f64))) && (((((sp :: Image :: default () . size ()) . r#height) as f64) > ((0f64) as f64))))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
                    ) . apply_pin (_self) . get ())) != ((sp :: SharedString :: from (""))))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_has_focus }
                    ) . apply_pin (_self) . get ())) && ((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_accessible_action_default }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_43 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_47 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_43 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_49 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_background_44_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_43 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_47 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_43 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_49 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_43 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_47 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_43 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_49 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_2 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_2 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_2 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_2 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_2 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_3 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_3 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_3 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_3 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_3 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) && ((({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_pressed }
                        ) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             if ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 if ({
                                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                ) . apply_pin (_self) . get () {
                                     (4f64) as _ }
                                 else {
                                     0f64 }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_43_state = ({
                             * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (1f64 as f64)) {
                             (if ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                            ) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (2f64 as f64)) {
                                 (if ({
                                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                ) . apply_pin (_self) . get () {
                                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (4f64 as f64)) {
                                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                    )) as _ }
                                 else {
                                     if ({
                                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                    ) . apply_pin (_self) . get () {
                                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_43_state = ({
                             * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (1f64 as f64)) {
                             (if ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                            ) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (2f64 as f64)) {
                                 (if ({
                                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                ) . apply_pin (_self) . get () {
                                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3428896255f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3422576568f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((150994943f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (3f64 as f64)) {
                                     (if ({
                                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                    ) . apply_pin (_self) . get () {
                                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((3865103871f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858784184f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((2163866105f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (4f64 as f64)) {
                                         (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4284534271f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4278214584f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_43_state = ({
                             * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (1f64 as f64)) {
                             (if ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                            ) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_43_state . clone () as f64) , & (4f64 as f64)) {
                                     (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((603979776f64) as u32) , position : 1f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((1711276032f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                    ) as _ }
                                 else {
                                     if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((301989888f64) as u32) , position : 0.0833f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9058f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     ({
                                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                    ) . apply_pin (_self) . set ((! ({
                                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                                    ) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             ({
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_clicked }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! ((((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from (" "))))) || (((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\n")))))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     ({
                                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_47 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_49 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_53 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_47 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_49 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_53 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_47 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_49 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_43 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_53 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
                ) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_47 {
         r#image_47 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_47 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_47 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | sp :: Property :: link_two_way (({
                     * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 20f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 20f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , 20f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_47 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_47 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_47 :: FIELD_OFFSETS . r#image_47 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_47) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_47 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_47 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_47 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_47 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_47 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_47 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_49 {
         r#text_49 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_49 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_49 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text_color) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1.0766f64) as f64) * ((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_i_layout_46_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_49 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_49 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_text_49 :: FIELD_OFFSETS . r#text_49 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_49) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_49 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_49 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_49 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_49 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_49 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_49 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_53 {
         r#focusborder_53 : InnerFocusBorder_root_41 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_53 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_53 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_41 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_53 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             ({
                 InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
             + {
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
                 + {
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
                 + {
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
             + {
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
             + {
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
             + {
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_41 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_53 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
                 + {
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_53 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_53 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
                 + {
                     * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_53 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_53 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 ..= 1u32 => return {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_53 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_53 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_43 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_53 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
             + {
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#root_41 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_focusborder_53 :: FIELD_OFFSETS . r#focusborder_53 }
             + {
                 * & InnerFocusBorder_root_41 :: FIELD_OFFSETS . r#rectangle_42 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_53) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_53 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_53 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_53 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_53 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_53 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_53 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMenuItemBase_root_55 {
         r#root_55 : sp :: r#Empty , r#background_layer_56 : sp :: r#BasicBorderRectangle , r#touch_area_visibility_57 : sp :: r#Clip , r#touch_area_58 : sp :: r#TouchArea , r#rectangle_60 : sp :: r#Empty , r#image_63 : sp :: r#ImageItem , r#label_opacity_64 : sp :: r#Opacity , r#label_65 : sp :: r#SimpleText , r#root_55_background_layer_56_height : sp :: Property < sp :: LogicalLength > , r#root_55_background_layer_56_width : sp :: Property < sp :: LogicalLength > , r#root_55_current_background : sp :: Property < slint :: Brush > , r#root_55_current_foreground : sp :: Property < slint :: Brush > , r#root_55_default_foreground : sp :: Property < slint :: Brush > , r#root_55_entry : sp :: Property < sp :: MenuEntry > , r#root_55_height : sp :: Property < sp :: LogicalLength > , r#root_55_horizontal_padding : sp :: Property < sp :: LogicalLength > , r#root_55_icon_size : sp :: Property < sp :: LogicalLength > , r#root_55_image_63_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_55_image_63_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_55_image_63_y : sp :: Property < sp :: LogicalLength > , r#root_55_is_current : sp :: Property < bool > , r#root_55_label_65_font_metrics : sp :: Property < sp :: FontMetrics > , r#root_55_layout_59_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_55_layout_59_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_55_layout_59_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_55_layout_59_spacing : sp :: Property < sp :: LogicalLength > , r#root_55_separator_color : sp :: Property < slint :: Brush > , r#root_55_state : sp :: Property < i32 > , r#root_55_sub_menu_icon : sp :: Property < sp :: Image > , r#root_55_touch_area_58_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_55_touch_area_58_height : sp :: Property < sp :: LogicalLength > , r#root_55_touch_area_58_width : sp :: Property < sp :: LogicalLength > , r#root_55_width : sp :: Property < sp :: LogicalLength > , r#root_55_x : sp :: Property < sp :: LogicalLength > , r#root_55_y : sp :: Property < sp :: LogicalLength > , r#root_55_activate : sp :: Callback < (sp :: MenuEntry , sp :: Coord ,) , () > , r#root_55_clear_current : sp :: Callback < () , () > , r#root_55_set_current : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_text_61 > , repeater1 : sp :: Conditional < InnerComponent_image_66 > , repeater2 : sp :: Conditional < InnerComponent_rectangle_68 > , change_tracker0 : sp :: ChangeTracker , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMenuItemBase_root_55 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#checked) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#has_sub_menu) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#is_separator) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_background_layer_56_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_background_layer_56_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 9u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 9u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as f64) - (({
                         let r#tmp_root_55_label_65_font_metrics = ({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_label_65_font_metrics }
                        ) . apply_pin (_self) . get () ;
                         (((r#tmp_root_55_label_65_font_metrics . clone ()) . r#ascent) as f64) - (((r#tmp_root_55_label_65_font_metrics . clone ()) . r#descent) as f64) }
                    ) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_label_65_font_metrics }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                    ) . apply_pin (_self) . font_metrics (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (2usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) + (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_preferred_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                )) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1)) as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_66 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = ({
                                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_horizontal_padding }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#end = ({
                                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_horizontal_padding }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : ({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_spacing }
                            ) . apply_pin (_self) . get () . get () as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (2usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) + (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_preferred_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                )) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1)) as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_66 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , ({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_spacing }
                        ) . apply_pin (_self) . get () . get () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = ({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_horizontal_padding }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct . r#end = ({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_horizontal_padding }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (2usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = ({
                                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_preferred_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                            ))) as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 10u32 - 1)) as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_66 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_spacing }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_is_current }
                    ) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ! (({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                        ) . apply_pin (_self) . get ()) . r#enabled {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_absolute_position }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((r#parent_position . clone ()) . r#x) as f64) + ((0f64) as f64)) as _ ;
                             the_struct . r#y = ((((r#parent_position . clone ()) . r#y) as f64) + ((0f64) as f64)) as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#background_layer_56 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_current_background }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (! (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#is_separator))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#enabled) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Move)))) && ((! ({
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_is_current }
                            ) . apply_pin (_self) . get ())) {
                                 ({
                                     ({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_set_current }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 if (((((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) && (((({
                                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                                ) . apply_pin (_self) . get ()) . r#has_sub_menu)))) && (((({
                                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                                ) . apply_pin (_self) . get ()) . r#enabled)) {
                                     ({
                                         ({
                                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_activate }
                                        ) . apply_pin (_self) . call (& (({
                                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                                        ) . apply_pin (_self) . get () as _ , (({
                                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_absolute_position }
                                        ) . apply_pin (_self) . get ()) . r#y as _ ,)) }
                                    ) ;
                                     }
                                 else {
                                     if (((((((((((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Up)))) && ((((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) && ((((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) < ((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                                    ) . apply_pin (_self) . get () . get ()) as f64))))) && ((((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) && ((((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) < ((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                                    ) . apply_pin (_self) . get () . get ()) as f64))))) && (((({
                                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                                    ) . apply_pin (_self) . get ()) . r#enabled)) {
                                         ({
                                             ({
                                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_activate }
                                            ) . apply_pin (_self) . call (& (({
                                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                                            ) . apply_pin (_self) . get () as _ , (({
                                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_touch_area_58_absolute_position }
                                            ) . apply_pin (_self) . get ()) . r#y as _ ,)) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_55_label_65_font_metrics = ({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_label_65_font_metrics }
                        ) . apply_pin (_self) . get () ;
                         (((r#tmp_root_55_label_65_font_metrics . clone ()) . r#ascent) as f64) - (((r#tmp_root_55_label_65_font_metrics . clone ()) . r#descent) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#icon) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1f64) as f64) * ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_opacity_64 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (0.5f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_current_foreground }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         ({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_default_foreground }
                        ) . apply_pin (_self) . get () }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_background_layer_56_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                    ) . apply_pin (_self) . get ()) . r#title) as _ }
                ) ;
                 }
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#background_layer_56 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#background_layer_56 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     if ((! ({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) && ((({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_is_current }
                    ) . apply_pin (_self) . get ())) {
                         (({
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_clear_current }
                        ) . apply_pin (_self) . call (& ())) ;
                         }
                     else {
                         {
                             }
                         }
                     }
                 ;
                 }
            ) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_61 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_66 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + (((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + (((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_h }
                ) . apply_pin (_self) . get ())))))) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + (((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + (((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_v }
                ) . apply_pin (_self) . get ())))))) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_61 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_66 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_61 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_66 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerMenuItemBase_root_55 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => ((((((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 0f64 as sp :: Coord ,) , 6u32 => ((((((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 0f64 as sp :: Coord ,) , 9u32 => ({
                     let r#tmp_root_55_label_65_font_metrics = ({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_label_65_font_metrics }
                    ) . apply_pin (_self) . get () ;
                     (((r#tmp_root_55_label_65_font_metrics . clone ()) . r#ascent) as f64) - (((r#tmp_root_55_label_65_font_metrics . clone ()) . r#descent) as f64) }
                 as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_image_63_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 10u32 => ((((((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as sp :: Coord , ({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 10u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (10u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((({
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                ) . apply_pin (_self) . get ()) . r#title) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_61 {
         r#text_61 : sp :: r#SimpleText , r#text_61_min_height : sp :: Property < sp :: LogicalLength > , r#text_61_min_width : sp :: Property < sp :: LogicalLength > , r#text_61_preferred_height : sp :: Property < sp :: LogicalLength > , r#text_61_preferred_width : sp :: Property < sp :: LogicalLength > , r#text_61_x : sp :: Property < sp :: LogicalLength > , r#text_61_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_61 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_61 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerFluentPalette_112 :: FIELD_OFFSETS . r#foreground . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("✓")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                     + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as f64) - ((({
                         * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                     + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("✓")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_61 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_61 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_text_61 :: FIELD_OFFSETS . r#text_61 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_61) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_61 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_61 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_61 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_61 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 8u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_61 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_61 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_66 {
         r#image_66 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_66 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_66 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                     + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_background_layer_56_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_sub_menu_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((0f64) as f64)) as f64) - ((0f64) as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [4usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_66 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_66 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_66 :: FIELD_OFFSETS . r#image_66 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_66) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_66 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_66 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_66 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_66 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_66 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_66 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_68 {
         r#rectangle_68 : sp :: r#Rectangle , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_68 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_68 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_68 :: FIELD_OFFSETS . r#rectangle_68 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_separator_color) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_rectangle_68 :: FIELD_OFFSETS . r#rectangle_68 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_rectangle_68 :: FIELD_OFFSETS . r#rectangle_68 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (1f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((1f64) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_rectangle_68 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_55 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_68 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_68 :: FIELD_OFFSETS . r#rectangle_68 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_68) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_68 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_rectangle_68 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_68 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_68 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_68 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_68 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMenuItem_root_70 {
         r#root_70 : sp :: r#Empty , r#base_72 : InnerMenuItemBase_root_55 , r#root_70_empty_71_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_70_empty_71_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_70_empty_71_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_70_height : sp :: Property < sp :: LogicalLength > , r#root_70_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_70_width : sp :: Property < sp :: LogicalLength > , r#root_70_x : sp :: Property < sp :: LogicalLength > , r#root_70_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMenuItem_root_70 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMenuItem_root_70 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMenuItemBase_root_55 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_72 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 1u32 - 1 , tree_index_of_first_child + 2u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + (((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + (((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + ((({
                                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                             + {
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_h }
                            ) . apply_pin (_self) . get ()))))))) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + ((({
                             InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                         + {
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_h }
                        ) . apply_pin (_self) . get ()))))))) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 5f64 as _ ;
                         the_struct . r#end = 5f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + ((({
                             InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                         + {
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_v }
                        ) . apply_pin (_self) . get ()))))))) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 5f64 as _ ;
                         the_struct . r#end = 5f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_4 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_4 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_4 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (40f64 as sp :: Coord) . max ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + (((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + (((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + ((({
                                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                             + {
                                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))))))) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_4 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_4 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_4 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#background_layer_56 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_current_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((167772160f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_current_foreground }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_default_foreground }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1.0766f64) as f64) * ((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_horizontal_padding }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (11f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_separator_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_spacing }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_sub_menu_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_horizontal_padding }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_icon_size }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_sub_menu_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMenuItemBase_root_55 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_72 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_72 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (40f64 as sp :: Coord) . max ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + ((({
                             InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                         + {
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_v }
                        ) . apply_pin (_self) . get ())))))))) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_72 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_72 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((((((({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , ({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 5f64 as sp :: Coord ,) , 2u32 ..= 11u32 => return {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . item_geometry (index - 2u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . accessible_role (0) , 2u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . accessible_role (index - 2u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (2u32 ..= 11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . accessible_string_property (index - 2u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (2u32 ..= 11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . accessibility_action (index - 2u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 2u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 2u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_72 }
                 . apply_pin (_self) . item_element_infos (index - 2u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerPopupMenuImpl_root_98 {
         r#root_98 : sp :: r#WindowItem , r#focus_scope_100 : sp :: r#FocusScope , r#frame_shadow_101 : sp :: r#BoxShadow , r#frame_102 : sp :: r#BasicBorderRectangle , r#frame_clip_103 : sp :: r#Clip , r#sub_menu_107 : sp :: r#ContextMenu , r#root_98_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_98_current_highlight : sp :: Property < i32 > , r#root_98_current_highlight_y_pos : sp :: Property < sp :: LogicalLength > , r#root_98_current_open : sp :: Property < i32 > , r#root_98_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_98_frame_102_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_98_layout_104_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_98_layout_104_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_98_layout_104_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_98_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_98_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_98_optimized_open_sub_menu_after_timeout_99_interval : sp :: Property < i64 > , r#root_98_optimized_open_sub_menu_after_timeout_99_running : sp :: Property < bool > , r#root_98_sub_menu_107_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_98_sub_menu_107_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_98_activated : sp :: Callback < (sp :: MenuEntry ,) , () > , r#root_98_close : sp :: Callback < () , () > , r#root_98_optimized_open_sub_menu_after_timeout_99_triggered : sp :: Callback < () , () > , r#root_98_sub_menu : sp :: Callback < (sp :: MenuEntry ,) , sp :: ModelRc < sp :: MenuEntry > > , repeater0 : sp :: Repeater < InnerComponent_menuitem_105 > , change_tracker0 : sp :: ChangeTracker , change_tracker1 : sp :: ChangeTracker , timer0 : sp :: Timer , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_98 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerPopupMenuImpl_root_98 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_absolute_position }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((r#parent_position . clone ()) . r#x) as f64) + ((0f64) as f64)) as _ ;
                             the_struct . r#y = ((((r#parent_position . clone ()) . r#y) as f64) + ((0f64) as f64)) as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
            ) . apply_pin (_self) . set ({
                 (((- 1f64) as i32)) as i32 }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_open }
            ) . apply_pin (_self) . set ({
                 (((- 1f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! []))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_frame_102_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_5 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_5 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_5 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (280f64 as sp :: Coord) . max ((({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_5 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_5 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_5 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_105 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 1f64 as _ ;
                                 the_struct . r#end = 1f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_105 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 1f64 as _ ;
                             the_struct . r#end = 1f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_105 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 1f64 as _ ;
                             the_struct . r#end = 1f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((sp :: Item :: layout_info (({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())))) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_frame_102_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((sp :: Item :: layout_info (({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())))) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))))))) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_interval }
            ) . apply_pin (_self) . set ({
                 (500f64) as i64 }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_triggered }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running }
                            ) . apply_pin (_self) . set (false as _) ;
                             if ((({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                            ) . apply_pin (_self) . get ()) as f64) >= ((0f64) as f64) {
                                 ({
                                     if (match & ({
                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                    ) . apply_pin (_self) . get () {
                                         x => {
                                             let index = (({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                            ) . apply_pin (_self) . get ()) as usize ;
                                             x . row_data_tracked (index) . unwrap_or_default () }
                                         }
                                    ) . r#has_sub_menu {
                                         ({
                                             _self . r#fn_activate (match & ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                            ) . apply_pin (_self) . get () {
                                                 x => {
                                                     let index = (({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . get ()) as usize ;
                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                 }
                                             as _ , ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight_y_pos }
                                            ) . apply_pin (_self) . get () . get () as _ , ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                            ) . apply_pin (_self) . get () as _) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_open }
                                            ) . apply_pin (_self) . set ((((- 1f64)) as i32) as _) ;
                                             ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                                            ) . apply_pin (_self) . r#close (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) }
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu_107_absolute_position }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((r#parent_position . clone ()) . r#x) as f64) + ((0f64) as f64)) as _ ;
                             the_struct . r#y = ((((r#parent_position . clone ()) . r#y) as f64) + ((0f64) as f64)) as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression2 = {
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running }
                                ) . apply_pin (_self) . set (false as _) ;
                                 let r#return_check_merge2 = if (((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\u{f700}"))) {
                                     ((false , {
                                         if ((({
                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                        ) . apply_pin (_self) . get ()) as f64) < ((1f64) as f64) {
                                             ({
                                                 ({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                ) . apply_pin (_self) . set ((((((match & ({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                ) . apply_pin (_self) . get () {
                                                     x => {
                                                         x . model_tracker () . track_row_count_changes () ;
                                                         x . row_count () as i32 }
                                                     }
                                                ) as f64) - ((1f64) as f64))) as i32) as _) }
                                            ) ;
                                             }
                                         else {
                                             if (match & ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                            ) . apply_pin (_self) . get () {
                                                 x => {
                                                     let index = ((((((({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . get ()) as f64) - ((1f64) as f64))) as i32)) as usize ;
                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                 }
                                            ) . r#is_separator {
                                                 ({
                                                     ({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . set ((((({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . get ()) as f64) - (((2f64) as i32) as f64)) as _) }
                                                ) ;
                                                 }
                                             else {
                                                 {
                                                     ({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . set ((((({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . get ()) as f64) - (((1f64) as i32) as f64)) as _) }
                                                 }
                                             }
                                         ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if (((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\u{f701}"))) {
                                         ((false , {
                                             if ((({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                            ) . apply_pin (_self) . get ()) as f64) >= ((((match & ({
                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                            ) . apply_pin (_self) . get () {
                                                 x => {
                                                     x . model_tracker () . track_row_count_changes () ;
                                                     x . row_count () as i32 }
                                                 }
                                            ) as f64) - ((1f64) as f64)) as f64) {
                                                 ({
                                                     ({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . set (((0f64) as i32) as _) }
                                                ) ;
                                                 }
                                             else {
                                                 if (match & ({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                ) . apply_pin (_self) . get () {
                                                     x => {
                                                         let index = ((((((({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . get ()) as f64) + ((1f64) as f64))) as i32)) as usize ;
                                                         x . row_data_tracked (index) . unwrap_or_default () }
                                                     }
                                                ) . r#is_separator {
                                                     ({
                                                         ({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . set ((((({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . get ()) as f64) + (((2f64) as i32) as f64)) as _) }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         ({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . set ((((({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . get ()) as f64) + (((1f64) as i32) as f64)) as _) }
                                                     }
                                                 }
                                             ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                     else {
                                         if (((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\n"))) {
                                             ((false , {
                                                 if ((((((({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                ) . apply_pin (_self) . get ()) as f64) >= ((0f64) as f64))) && ((((({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                ) . apply_pin (_self) . get ()) as f64) < ((match & ({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                ) . apply_pin (_self) . get () {
                                                     x => {
                                                         x . model_tracker () . track_row_count_changes () ;
                                                         x . row_count () as i32 }
                                                     }
                                                ) as f64))))) && (((match & ({
                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                ) . apply_pin (_self) . get () {
                                                     x => {
                                                         let index = (({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . get ()) as usize ;
                                                         x . row_data_tracked (index) . unwrap_or_default () }
                                                     }
                                                ) . r#enabled)) {
                                                     ({
                                                         _self . r#fn_activate (match & ({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                        ) . apply_pin (_self) . get () {
                                                             x => {
                                                                 let index = (({
                                                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                                ) . apply_pin (_self) . get ()) as usize ;
                                                                 x . row_data_tracked (index) . unwrap_or_default () }
                                                             }
                                                         as _ , ({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight_y_pos }
                                                        ) . apply_pin (_self) . get () . get () as _ , ({
                                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                        ) . apply_pin (_self) . get () as _) }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         }
                                                     }
                                                 ;
                                                 sp :: r#EventResult :: r#Accept }
                                             ,)) as _ }
                                         else {
                                             if ! ((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\u{f703}")))) {
                                                 ({
                                                     if (((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\u{f702}"))) {
                                                         ({
                                                             ({
                                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_close }
                                                            ) . apply_pin (_self) . call (& ()) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     (true , sp :: r#EventResult :: r#Reject ,) }
                                                ) as _ }
                                             else {
                                                 (false , {
                                                     if ((((((((({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . get ()) as f64) >= ((0f64) as f64))) && ((((({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                    ) . apply_pin (_self) . get ()) as f64) < ((match & ({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                    ) . apply_pin (_self) . get () {
                                                         x => {
                                                             x . model_tracker () . track_row_count_changes () ;
                                                             x . row_count () as i32 }
                                                         }
                                                    ) as f64))))) && (((match & ({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                    ) . apply_pin (_self) . get () {
                                                         x => {
                                                             let index = (({
                                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                            ) . apply_pin (_self) . get ()) as usize ;
                                                             x . row_data_tracked (index) . unwrap_or_default () }
                                                         }
                                                    ) . r#has_sub_menu)))) && (((match & ({
                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                    ) . apply_pin (_self) . get () {
                                                         x => {
                                                             let index = (({
                                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                            ) . apply_pin (_self) . get ()) as usize ;
                                                             x . row_data_tracked (index) . unwrap_or_default () }
                                                         }
                                                    ) . r#enabled)) {
                                                         ({
                                                             _self . r#fn_activate (match & ({
                                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                                            ) . apply_pin (_self) . get () {
                                                                 x => {
                                                                     let index = (({
                                                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                                    ) . apply_pin (_self) . get ()) as usize ;
                                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                                 }
                                                             as _ , ({
                                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight_y_pos }
                                                            ) . apply_pin (_self) . get () . get () as _ , ({
                                                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight }
                                                            ) . apply_pin (_self) . get () as _) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     sp :: r#EventResult :: r#Accept }
                                                 ,) }
                                             }
                                         }
                                     }
                                 ;
                                 if (r#return_check_merge2 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge2 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression2 . clone ()) . 1 {
                                 ((r#returned_expression2 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression2 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
                 + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((1107296256f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((603979776f64) as u32) }
                    ) . color ()) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032284f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294638330f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_112 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_112 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_open }
                            ) . apply_pin (_self) . set ((((- 1f64)) as i32) as _) ;
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_activated }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) ;
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_close }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
             + sp :: r#ContextMenu :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_98 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let window_adapter = & _self . globals . get () . unwrap () . window_adapter_impl () ;
                             let entries = ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu_107_entries }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_entries }
                                ) . apply_pin (_self) . set (entries . clone ()) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                                        ) . apply_pin (_self) . sub_menu . call (entry) }
                                     else {
                                         :: core :: default :: Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_activated }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                                        ) . apply_pin (_self) . activated . call (entry) }
                                     else {
                                         :: core :: default :: Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_close }
                                ) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = ({
                                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                                    ) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             if let Some (current_id) = ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                            ) . apply_pin (_self) . popup_id . take () {
                                 sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . close_popup (current_id) ;
                                 }
                             let id = sp :: WindowInner :: from_pub (window_adapter . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true ,) ;
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                            ) . apply_pin (_self) . popup_id . set (Some (id)) ;
                             InnerPopupMenuImpl_root_98 :: user_init (popup_instance_vrc) ;
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#sub_menu) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_interval }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
             + sp :: r#ContextMenu :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . r#fn_focus () ;
             {
                 }
             ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_interval }
                ) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     _self . update_timers () }
                 ;
                 }
            ) ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker1 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running }
                ) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     _self . update_timers () }
                 ;
                 }
            ) ;
             _self . update_timers () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_105 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_105 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_105 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         fn update_timers (self : :: core :: pin :: Pin < & Self >) {
             let _self = self ;
             if ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running }
            ) . apply_pin (_self) . get () {
                 let interval = :: core :: time :: Duration :: from_millis (({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_interval }
                ) . apply_pin (_self) . get () as u64) ;
                 if ! self . timer0 . running () || interval != self . timer0 . interval () {
                     let self_weak = self . self_weak . get () . unwrap () . clone () ;
                     self . timer0 . start (sp :: TimerMode :: Repeated , interval , move || {
                         if let Some (self_rc) = self_weak . upgrade () {
                             let _self = self_rc . as_pin_ref () ;
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_triggered }
                            ) . apply_pin (_self) . call (& ()) }
                         }
                    ) ;
                     }
                 }
             else {
                 self . timer0 . stop () ;
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_activate (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: MenuEntry , arg_1 : sp :: Coord , arg_2 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 , arg_2 ,) ;
             ({
                 ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running }
                ) . apply_pin (_self) . set (false as _) ;
                 if (args . 0 . clone ()) . r#has_sub_menu {
                     (if ((! sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_open }
                    ) . apply_pin (_self) . get () as f64) , & (args . 2 . clone () as f64)))) || ((! ({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                    ) . apply_pin (_self) . r#is_open (& _self . globals . get () . unwrap () . window_adapter_impl () , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)))) {
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_open }
                            ) . apply_pin (_self) . set (args . 2 . clone () as _) ;
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu_107_entries }
                            ) . apply_pin (_self) . set (({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) as _) ;
                             ({
                                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
                             + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) . call (& ({
                                 let mut the_struct = slint :: LogicalPosition :: default () ;
                                 the_struct . r#x = ({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#y = (((args . 1 . clone ()) as f64) - (((({
                                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_sub_menu_107_absolute_position }
                                ) . apply_pin (_self) . get ()) . r#y) as f64)) as _ ;
                                 the_struct }
                             as _ ,)) }
                        ) ;
                         }
                     else {
                         {
                             }
                         }
                    ) ;
                     }
                 else {
                     {
                         ({
                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_open }
                        ) . apply_pin (_self) . set (((- 1f64) as i32) as _) ;
                         ({
                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_activated }
                        ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) ;
                         ({
                             * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_close }
                        ) . apply_pin (_self) . call (& ()) }
                     }
                 }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_menuitem_105 {
         r#menuitem_105 : InnerMenuItem_root_70 , r#model_data : sp :: Property < sp :: MenuEntry > , r#model_index : sp :: Property < i32 > , r#menuitem_105_absolute_position : sp :: Property < slint :: LogicalPosition > , change_tracker0 : sp :: ChangeTracker , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_menuitem_105 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_98 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_menuitem_105 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMenuItem_root_70 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#menuitem_105 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105_absolute_position }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#menuitem_105 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#menuitem_105 . tree_index . get ())) . map_to_window (:: core :: default :: Default :: default ())) ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ((((r#parent_position . clone ()) . r#x) as f64) + ((1f64) as f64)) as _ ;
                             the_struct . r#y = ((((r#parent_position . clone ()) . r#y) as f64) + ((({
                                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                             + {
                                 * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_y }
                            ) . apply_pin (_self) . get () . get ()) as f64)) as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_activate }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | x . as_pin_ref () . r#fn_activate (args . 0 . clone () as _ , args . 1 . clone () as _ , ({
                                     * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
                                ) . apply_pin (_self) . get () as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_clear_current }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set ((((- 1f64)) as i32) as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_entry }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + ({
                             * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_is_current }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ApproxEq :: < f64 > :: approx_eq (& ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () as f64) , & (({
                         * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64))) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_set_current }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 1u32 - 1) , true , sp :: FocusReason :: Programmatic) ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (({
                                     * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
                                ) . apply_pin (_self) . get () as _)) ;
                                 }
                             ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_optimized_open_sub_menu_after_timeout_99_running) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (true as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((1f64) as f64)) as f64) - ((1f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_layout_104_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMenuItem_root_70 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#menuitem_105 }
             . apply_pin (x)) ,) ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                 + {
                     * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_is_current }
                ) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     if ({
                         InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                     + {
                         InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                     + {
                         * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_is_current }
                    ) . apply_pin (_self) . get () {
                         ({
                             let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_current_highlight_y_pos) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (sp :: LogicalLength :: new (((((({
                                 * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105_absolute_position }
                            ) . apply_pin (_self) . get ()) . r#y) as f64) - ((((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98_absolute_position) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) . r#y) as f64)) as sp :: Coord) as _)) ;
                             }
                        ) ;
                         }
                     else {
                         {
                             }
                         }
                     }
                 ;
                 }
            ) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_empty_71_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                     + {
                         * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (40f64 as sp :: Coord) . max ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + (((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + ((({
                             InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                         + {
                             InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
                         + {
                             * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55_layout_59_layoutinfo_v }
                        ) . apply_pin (_self) . get ())))))))) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((((_self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((1f64) as f64)) as f64) - ((1f64) as f64)) as sp :: Coord , 1f64 as sp :: Coord , ({
                     InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
                 + {
                     * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 ..= 11u32 => return {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_105 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_menuitem_105 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_98 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_98 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             12usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 6u32 , parent_index : 3u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 9u32 , parent_index : 5u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 11u32 , parent_index : 5u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 11u32 , parent_index : 6u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 12u32 , parent_index : 7u32 , item_array_index : 8u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_menuitem_105 , sp :: ItemVTable , sp :: AllowPin > ;
             9usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 * & InnerMenuItem_root_70 :: FIELD_OFFSETS . r#root_70 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#root_55 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#background_layer_56 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_visibility_57 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#touch_area_58 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#rectangle_60 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_opacity_64 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#image_63 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#menuitem_105 }
             + {
                 InnerMenuItem_root_70 :: FIELD_OFFSETS . r#base_72 }
             + {
                 * & InnerMenuItemBase_root_55 :: FIELD_OFFSETS . r#label_65 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_menuitem_105) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_menuitem_105 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_menuitem_105 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_menuitem_105 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_menuitem_105 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_menuitem_105 {
         type Data = sp :: MenuEntry ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_menuitem_105 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_menuitem_105 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerPopupMenuImpl_root_98 {
         fn new (globals : sp :: Rc < SharedGlobals >) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = globals ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             7usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 6u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 5u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerPopupMenuImpl_root_98 , sp :: ItemVTable , sp :: AllowPin > ;
             6usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#root_98 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#focus_scope_100 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#sub_menu_107 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_shadow_101 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_102 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_98 :: FIELD_OFFSETS . r#frame_clip_103 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerPopupMenuImpl_root_98) ;
         }
     ;
     impl sp :: PinnedDrop for InnerPopupMenuImpl_root_98 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerPopupMenuImpl_root_98 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerPopupMenuImpl_root_98 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerPopupMenuImpl_root_98 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_73 : sp :: r#WindowItem , r#empty_75 : sp :: r#Empty , r#empty_85 : sp :: r#Empty , r#text_86 : sp :: r#SimpleText , r#input1_87 : InnerTextEdit_root_1 , r#input2_88 : InnerTextEdit_root_1 , r#button_89 : InnerButton_root_43 , r#root_73_empty_74_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_73_empty_74_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_73_empty_74_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_73_empty_75_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_73_empty_75_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_73_empty_75_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_73_empty_85_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_73_empty_85_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_73_empty_85_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_73_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_73_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , repeater0 : sp :: Repeater < InnerComponent_rectangle_76 > , repeater1 : sp :: Conditional < InnerComponent_rectangle_90 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAppWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (InnerAppLogic :: FIELD_OFFSETS . r#menu_items . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get ()) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (InnerAppLogic :: FIELD_OFFSETS . r#popup_visible . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get ()) as _ }
                 }
            ) ;
             InnerTextEdit_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#input1_87 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 9u32 - 1) ;
             InnerTextEdit_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#input2_88 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 7u32 - 1 , tree_index_of_first_child + 43u32 - 1) ;
             InnerButton_root_43 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_89 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 8u32 - 1 , tree_index_of_first_child + 77u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerTheme :: FIELD_OFFSETS . r#background_color . apply_pin (_self . globals . get () . unwrap () . global_Theme . as_ref ()) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (2usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_90 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 10f64 as _ ;
                                 the_struct . r#end = 10f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (2usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_90 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (2usize + _self . repeater1 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_90 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_76 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 8f64 as _ ;
                                 the_struct . r#end = 8f64 as _ ;
                                 the_struct }
                             as _ , r#size : (((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as _ , r#spacing : 8f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_76 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_76 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1)) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let r#layout_info_1 = {
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                         the_struct . r#max_percent = 100f64 as _ ;
                                         the_struct . r#min = 0f64 as _ ;
                                         the_struct . r#min_percent = 0f64 as _ ;
                                         the_struct . r#preferred = 0f64 as _ ;
                                         the_struct . r#stretch = 1f64 as _ ;
                                         the_struct }
                                     ;
                                     {
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                                         the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                                         the_struct . r#min = (r#layout_info_1 . clone ()) . r#min as _ ;
                                         the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                                         the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                                         the_struct . r#stretch = 1f64 as _ ;
                                         the_struct }
                                     }
                                )) + (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let r#layout_info_1 = {
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                         the_struct . r#max_percent = 100f64 as _ ;
                                         the_struct . r#min = 0f64 as _ ;
                                         the_struct . r#min_percent = 0f64 as _ ;
                                         the_struct . r#preferred = 0f64 as _ ;
                                         the_struct . r#stretch = 1f64 as _ ;
                                         the_struct }
                                     ;
                                     {
                                         let mut the_struct = sp :: LayoutInfo :: default () ;
                                         the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                                         the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                                         the_struct . r#min = (r#layout_info_1 . clone ()) . r#min as _ ;
                                         the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                                         the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                                         the_struct . r#stretch = 1f64 as _ ;
                                         the_struct }
                                     }
                                )) + (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                                 + {
                                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 20f64 as _ ;
                             the_struct . r#end = 20f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let r#layout_info_0 = {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info_0 . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info_0 . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info_0 . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                            )) + (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = 100f64 as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let r#layout_info_0 = {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info_0 . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info_0 . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info_0 . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                            )) + (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = 100f64 as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                             + {
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 100f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 100f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 20f64 as _ ;
                         the_struct . r#end = 20f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 5u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let r#layout_info_1 = {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info_1 . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                            )) + (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let r#layout_info_1 = {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info_1 . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                            )) + (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                             + {
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 20f64 as _ ;
                         the_struct . r#end = 20f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())))) + ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())))) + ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerStrings :: FIELD_OFFSETS . r#app_name . apply_pin (_self . globals . get () . unwrap () . global_Strings . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294967295f64) as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((match & InnerAppLogic :: FIELD_OFFSETS . r#menu_items . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get () {
                         x => {
                             let index = (InnerAppLogic :: FIELD_OFFSETS . r#current_menu . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get ()) as usize ;
                             x . row_data_tracked (index) . unwrap_or_default () }
                         }
                    ) . r#name) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as f64) - ((20f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as f64) - ((20f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as f64) - ((20f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             InnerAppLogic :: FIELD_OFFSETS . r#submit . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . call (& ((match & InnerAppLogic :: FIELD_OFFSETS . r#menu_items . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get () {
                                 x => {
                                     let index = (InnerAppLogic :: FIELD_OFFSETS . r#current_menu . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get ()) as usize ;
                                     x . row_data_tracked (index) . unwrap_or_default () }
                                 }
                            ) . r#name as _ , ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                             + {
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ , ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                             + {
                                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerStrings :: FIELD_OFFSETS . r#submit_button . apply_pin (_self . globals . get () . unwrap () . global_Strings . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                    ) . apply_pin (_self) . get () [6usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerTextEdit_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#input1_87 }
             . apply_pin (x)) ,) ;
             InnerTextEdit_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#input2_88 }
             . apply_pin (x)) ,) ;
             InnerButton_root_43 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_89 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_76 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_90 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_89 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 2u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_76 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_90 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_89 }
                     . apply_pin (_self) . subtree_range (dyn_index - 2u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_76 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_90 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_89 }
                     . apply_pin (_self) . subtree_component (dyn_index - 2u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , 10f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , 10f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 5u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , (((((((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as f64) - ((20f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord , 20f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 6u32 => (40f64 as sp :: Coord , (((((((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as f64) - ((20f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord , 20f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 7u32 => (40f64 as sp :: Coord , (((((((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as f64) - ((20f64) as f64)) as f64) - ((20f64) as f64)) as sp :: Coord , 20f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 8u32 => (40f64 as sp :: Coord , 100f64 as sp :: Coord , 20f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_85_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord ,) , 9u32 ..= 42u32 => return {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . item_geometry (index - 9u32 + 1) , 43u32 ..= 76u32 => return {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . item_geometry (index - 43u32 + 1) , 77u32 ..= 83u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . item_geometry (index - 77u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5u32 => sp :: r#AccessibleRole :: r#Text , 6u32 => sp :: r#AccessibleRole :: r#TextInput , 7u32 => sp :: r#AccessibleRole :: r#TextInput , 8u32 => sp :: r#AccessibleRole :: r#Button , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . accessible_role (0) , 9u32 ..= 42u32 => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . accessible_role (index - 9u32 + 1) , 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . accessible_role (0) , 43u32 ..= 76u32 => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . accessible_role (index - 43u32 + 1) , 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . accessible_role (0) , 77u32 ..= 83u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . accessible_role (index - 77u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (5u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (6u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (6u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (6u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (6u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (7u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (7u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (7u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (7u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
                 + {
                     * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (8u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
                ) . apply_pin (_self) . get ()) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (9u32 ..= 42u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . accessible_string_property (index - 9u32 + 1 , what) , (7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (43u32 ..= 76u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . accessible_string_property (index - 43u32 + 1 , what) , (8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (77u32 ..= 83u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . accessible_string_property (index - 77u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (8u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
                     + {
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (9u32 ..= 42u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . accessibility_action (index - 9u32 + 1 , action) , (7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (43u32 ..= 76u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . accessibility_action (index - 43u32 + 1 , action) , (8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (77u32 ..= 83u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . accessibility_action (index - 77u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 8u32 => sp :: SupportedAccessibilityAction :: r#Default , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 9u32 ..= 42u32 => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 9u32 + 1) , 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 43u32 ..= 76u32 => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 43u32 + 1) , 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 77u32 ..= 83u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 77u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 ..= 42u32 => {
                     * & Self :: FIELD_OFFSETS . r#input1_87 }
                 . apply_pin (_self) . item_element_infos (index - 9u32 + 1) , 43u32 ..= 76u32 => {
                     * & Self :: FIELD_OFFSETS . r#input2_88 }
                 . apply_pin (_self) . item_element_infos (index - 43u32 + 1) , 77u32 ..= 83u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_89 }
                 . apply_pin (_self) . item_element_infos (index - 77u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_76 {
         r#rectangle_76 : sp :: r#Rectangle , r#toucharea_83 : sp :: r#TouchArea , r#model_data : sp :: Property < r#MenuItem > , r#model_index : sp :: Property < i32 > , r#rectangle_76_x : sp :: Property < sp :: LogicalLength > , repeater0 : sp :: Conditional < InnerComponent_image_77 > , repeater1 : sp :: Conditional < InnerComponent_image_79 > , repeater2 : sp :: Conditional < InnerComponent_image_81 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_76 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_76 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
                         * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#name)) == ((sp :: SharedString :: from ("Home"))))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
                         * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#name)) == ((sp :: SharedString :: from ("Profile"))))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
                         * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#name)) == ((sp :: SharedString :: from ("Settings"))))) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#rectangle_76 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if sp :: ApproxEq :: < f64 > :: approx_eq (& (InnerAppLogic :: FIELD_OFFSETS . r#current_menu . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get () as f64) , & (({
                         * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64)) {
                         (InnerTheme :: FIELD_OFFSETS . r#accent_color . apply_pin (_self . globals . get () . unwrap () . global_Theme . as_ref ()) . get ()) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((0f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#rectangle_76_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_75_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#toucharea_83 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             InnerAppLogic :: FIELD_OFFSETS . r#menu_selected . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . call (& (({
                                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_index }
                            ) . apply_pin (_self) . get () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#toucharea_83 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#toucharea_83 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#toucharea_83 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_77 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_81 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#rectangle_76 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 60f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 60f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#rectangle_76 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 60f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 60f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_77 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_81 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_77 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_79 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerComponent_rectangle_76 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_81 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (60f64 as sp :: Coord , 60f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#rectangle_76_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 8f64 as sp :: Coord ,) , 4u32 => (60f64 as sp :: Coord , 60f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_77 {
         r#image_77 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_77 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_77 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (24f64 as sp :: Coord , 24f64 as sp :: Coord , 18f64 as sp :: Coord , 18f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_77 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_77 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_77 :: FIELD_OFFSETS . r#image_77 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_77) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_77 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_77 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_77 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_77 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 1u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_77 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_77 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_79 {
         r#image_79 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_79 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_79 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_6 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (24f64 as sp :: Coord , 24f64 as sp :: Coord , 18f64 as sp :: Coord , 18f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_79 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_79 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_79 :: FIELD_OFFSETS . r#image_79 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_79) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_79 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_79 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_79 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_79 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 2u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_79 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_79 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_81 {
         r#image_81 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_81 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_81 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_7 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (24f64 as sp :: Coord , 24f64 as sp :: Coord , 18f64 as sp :: Coord , 18f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_81 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_76 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_81 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_81 :: FIELD_OFFSETS . r#image_81 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_81) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_81 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_81 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_81 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_81 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_81 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_81 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerComponent_rectangle_76 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_76 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#rectangle_76 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#toucharea_83 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_76) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_76 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_rectangle_76 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_76 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_76 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_76 {
         type Data = r#MenuItem ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_rectangle_76 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_76 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_90 {
         r#rectangle_90 : sp :: r#Rectangle , r#empty_92 : sp :: r#Empty , r#rectangle_93 : sp :: r#Rectangle , r#text_95 : sp :: r#SimpleText , r#button_96 : InnerButton_root_43 , r#rectangle_90_empty_91_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_90_empty_91_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_90_empty_91_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_90_empty_92_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_90_empty_92_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_90_empty_92_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_90_empty_94_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_90_empty_94_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_90_empty_94_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_90_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_90 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_90 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_43 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_96 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 4u32 - 1 , tree_index_of_first_child + 5u32 - 1) ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) + ((({
                                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 100f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 100f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                             let cache = x . get () ;
                             * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                        ) . unwrap_or_default ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + ((({
                                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 200f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 200f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + ((({
                                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 100f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 100f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                                 + {
                                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : 100f64 as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                             + {
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 80f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 80f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                             + {
                                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_93 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerTheme :: FIELD_OFFSETS . r#primary_color . apply_pin (_self . globals . get () . unwrap () . global_Theme . as_ref ()) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294967295f64) as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerAppLogic :: FIELD_OFFSETS . r#popup_message . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (184f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             InnerAppLogic :: FIELD_OFFSETS . r#close_popup . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("OK")) as sp :: SharedString }
            ) ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (80f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_43 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_96 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_96 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layoutinfo_v }
                ) . apply_pin (_self) . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_96 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_96 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , (((((_self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , 10f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [4usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord ,) , 1u32 => (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_73_empty_74_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , ({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_91_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 8f64 as sp :: Coord ,) , 2u32 => (100f64 as sp :: Coord , 200f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 184f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 4u32 => (40f64 as sp :: Coord , 80f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90_empty_94_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 5u32 ..= 11u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . item_geometry (index - 5u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: r#AccessibleRole :: r#Text , 4u32 => sp :: r#AccessibleRole :: r#Button , 4u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_role (0) , 5u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_role (index - 5u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (InnerAppLogic :: FIELD_OFFSETS . r#popup_message . apply_pin (_self . globals . get () . unwrap () . global_AppLogic . as_ref ()) . get ()) , (4u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_text }
                ) . apply_pin (_self) . get ()) , (4u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (5u32 ..= 11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_string_property (index - 5u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (4u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
                     + {
                         * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (4u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (5u32 ..= 11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessibility_action (index - 5u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: SupportedAccessibilityAction :: r#Default , 4u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 5u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 5u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5u32 ..= 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . item_element_infos (index - 5u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_rectangle_90 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             12usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 9u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 4u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 4u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 5u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 5u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_90 , sp :: ItemVTable , sp :: AllowPin > ;
             9usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_90 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#empty_92 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#rectangle_93 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#text_95 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_90 :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_90) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_90 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_rectangle_90 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_90 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_90 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_90 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_90 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerAppWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             84usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 9u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 9u32 , parent_index : 2u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 43u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 77u32 , parent_index : 2u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 10u32 , parent_index : 6u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 13u32 , parent_index : 9u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 9u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 9u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 14u32 , parent_index : 10u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 13u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 19u32 , parent_index : 13u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 31u32 , parent_index : 13u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 18u32 , parent_index : 14u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 17u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 20u32 , parent_index : 15u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 21u32 , parent_index : 19u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 20u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 20u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 25u32 , parent_index : 20u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 28u32 , parent_index : 20u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 26u32 , parent_index : 23u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 27u32 , parent_index : 25u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 26u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 29u32 , parent_index : 24u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 30u32 , parent_index : 28u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 29u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 32u32 , parent_index : 16u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 33u32 , parent_index : 31u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 37u32 , parent_index : 32u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 37u32 , parent_index : 32u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 37u32 , parent_index : 32u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 40u32 , parent_index : 32u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 38u32 , parent_index : 35u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 39u32 , parent_index : 37u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 40u32 , parent_index : 38u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 41u32 , parent_index : 36u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 42u32 , parent_index : 40u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 41u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 44u32 , parent_index : 7u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 47u32 , parent_index : 43u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 77u32 , parent_index : 43u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 77u32 , parent_index : 43u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 48u32 , parent_index : 44u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 51u32 , parent_index : 47u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 53u32 , parent_index : 47u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 65u32 , parent_index : 47u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 52u32 , parent_index : 48u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 53u32 , parent_index : 51u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 54u32 , parent_index : 49u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 55u32 , parent_index : 53u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 59u32 , parent_index : 54u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 59u32 , parent_index : 54u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 59u32 , parent_index : 54u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 62u32 , parent_index : 54u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 60u32 , parent_index : 57u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 61u32 , parent_index : 59u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 62u32 , parent_index : 60u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 63u32 , parent_index : 58u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 64u32 , parent_index : 62u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 65u32 , parent_index : 63u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 66u32 , parent_index : 50u32 , item_array_index : 63u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 67u32 , parent_index : 65u32 , item_array_index : 64u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 71u32 , parent_index : 66u32 , item_array_index : 65u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 71u32 , parent_index : 66u32 , item_array_index : 66u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 71u32 , parent_index : 66u32 , item_array_index : 67u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 74u32 , parent_index : 66u32 , item_array_index : 68u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 72u32 , parent_index : 69u32 , item_array_index : 69u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 73u32 , parent_index : 71u32 , item_array_index : 70u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 74u32 , parent_index : 72u32 , item_array_index : 71u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 75u32 , parent_index : 70u32 , item_array_index : 72u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 76u32 , parent_index : 74u32 , item_array_index : 73u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 77u32 , parent_index : 75u32 , item_array_index : 74u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 81u32 , parent_index : 8u32 , item_array_index : 75u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 84u32 , parent_index : 8u32 , item_array_index : 76u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 84u32 , parent_index : 8u32 , item_array_index : 77u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 84u32 , parent_index : 77u32 , item_array_index : 78u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 77u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 77u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             79usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_73 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_75 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_85 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_86 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#root_43 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#i_focus_border_35 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#scroll_view_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_viewport_6 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_opacity_13 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_opacity_17 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_15 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_19 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_24 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_opacity_26 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_opacity_30 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_28 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_32 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input1_87 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#base_2 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#contextmenuinternal_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#placeholder_34 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#i_focus_border_35 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#scroll_view_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_visibility_8 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_visibility_21 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#flickable_viewport_6 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#text_input_7 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_9 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#vertical_bar_clip_10 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_opacity_13 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_opacity_17 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_14 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_15 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_16 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_18 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_19 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_20 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_22 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#horizontal_bar_clip_23 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#thumb_24 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#touch_area_25 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_opacity_26 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_opacity_30 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#up_scroll_button_27 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_28 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_29 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#down_scroll_button_31 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_opacity_32 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#input2_88 }
             + {
                 * & InnerTextEdit_root_1 :: FIELD_OFFSETS . r#icon_33 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_background_44 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_touch_area_51 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_focus_scope_52 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_89 }
             + {
                 * & InnerButton_root_43 :: FIELD_OFFSETS . r#i_border_45 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow > ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     struct SharedGlobals {
         global_AppLogic : :: core :: pin :: Pin < sp :: Rc < InnerAppLogic >> , global_Strings : :: core :: pin :: Pin < sp :: Rc < InnerStrings >> , global_Theme : :: core :: pin :: Pin < sp :: Rc < InnerTheme >> , global_FluentPalette_112 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_112 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_AppLogic : InnerAppLogic :: new () , global_Strings : InnerStrings :: new () , global_Theme : InnerTheme :: new () , global_FluentPalette_112 : InnerFluentPalette_112 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_AppLogic . clone () . init (& _self) ;
             _self . global_Strings . clone () . init (& _self) ;
             _self . global_Theme . clone () . init (& _self) ;
             _self . global_FluentPalette_112 . clone () . init (& _self) ;
             _self }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_5 : & 'static [u8] = :: core :: include_bytes ! ("E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-top-nav\\ui\\icons\\home.svg") ;
     static SLINT_EMBEDDED_RESOURCE_6 : & 'static [u8] = :: core :: include_bytes ! ("E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-top-nav\\ui\\icons\\profile.svg") ;
     static SLINT_EMBEDDED_RESOURCE_7 : & 'static [u8] = :: core :: include_bytes ! ("E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-top-nav\\ui\\icons\\settings.svg") ;
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" height=\"24px\" viewBox=\"0 -960 960 960\" width=\"24px\" fill=\"#e8eaed\">\n  <path d=\"m321-80-71-71 329-329-329-329 71-71 400 400L321-80Z\"/>\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 1C0 0.864583 0.0260417 0.735677 0.078125 0.613281C0.130208 0.490885 0.200521 0.385417 0.289062 0.296875C0.380208 0.205729 0.485677 0.134115 0.605469 0.0820312C0.72526 0.0273438 0.854167 0 0.992188 0H7.01172C7.14714 0 7.27474 0.0260417 7.39453 0.078125C7.51693 0.130208 7.6224 0.201823 7.71094 0.292969C7.79948 0.384115 7.86979 0.490885 7.92188 0.613281C7.97396 0.733073 8 0.860677 8 0.996094C8 1.10547 7.98438 1.20573 7.95312 1.29688C7.92448 1.38802 7.88021 1.47917 7.82031 1.57031L5.21875 5.35547C5.08073 5.55599 4.90365 5.71354 4.6875 5.82812C4.47396 5.94271 4.24479 6 4 6C3.75521 6 3.52474 5.94271 3.30859 5.82812C3.09505 5.71354 2.91927 5.55599 2.78125 5.35547L0.179688 1.57031C0.119792 1.48177 0.0742188 1.39193 0.0429688 1.30078C0.0143229 1.20964 0 1.10938 0 1Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 4C0 3.75521 0.0572917 3.52604 0.171875 3.3125C0.286458 3.09635 0.44401 2.91927 0.644531 2.78125L4.42969 0.179687C4.51823 0.119792 4.60807 0.0755208 4.69922 0.046875C4.79036 0.015625 4.89062 0 5 0C5.13542 0 5.26432 0.0260417 5.38672 0.078125C5.50911 0.130208 5.61458 0.201823 5.70312 0.292969C5.79427 0.38151 5.86589 0.485677 5.91797 0.605469C5.97266 0.72526 6 0.854167 6 0.992187L6 7.00781C6 7.14583 5.97266 7.27474 5.91797 7.39453C5.86589 7.51432 5.79427 7.61979 5.70313 7.71094C5.61458 7.79948 5.50911 7.86979 5.38672 7.92187C5.26432 7.97396 5.13542 8 5 8C4.79427 8 4.60417 7.9401 4.42969 7.82031L0.644531 5.21875C0.44401 5.08073 0.286458 4.90495 0.171875 4.69141C0.0572917 4.47526 0 4.24479 0 4Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 7.00781L0 0.992187C0 0.854167 0.0260417 0.72526 0.078125 0.605469C0.132812 0.485677 0.204427 0.38151 0.292969 0.292969C0.384115 0.201823 0.489583 0.130208 0.609375 0.078125C0.731771 0.0260417 0.861979 0 1 0C1.20573 0 1.39583 0.0598958 1.57031 0.179687L5.35547 2.78125C5.55859 2.92187 5.71615 3.09896 5.82813 3.3125C5.94271 3.52604 6 3.75521 6 4C6 4.24479 5.94271 4.47396 5.82813 4.6875C5.71615 4.90104 5.55859 5.07812 5.35547 5.21875L1.57031 7.82031C1.39583 7.9401 1.20573 8 1 8C0.861979 8 0.731771 7.97396 0.609375 7.92188C0.489583 7.86979 0.384115 7.79948 0.292969 7.71094C0.204427 7.61979 0.132813 7.51432 0.078125 7.39453C0.0260417 7.27474 0 7.14583 0 7.00781Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0.992188 6C0.854167 6 0.72526 5.97396 0.605469 5.92188C0.485677 5.86719 0.380208 5.79557 0.289062 5.70703C0.200521 5.61589 0.130208 5.51042 0.078125 5.39062C0.0260417 5.26823 0 5.13802 0 5C0 4.89062 0.0143229 4.79036 0.0429688 4.69922C0.0742188 4.60807 0.119792 4.51823 0.179688 4.42969L2.78125 0.644531C2.84896 0.545573 2.92839 0.458333 3.01953 0.382812C3.11068 0.307292 3.20833 0.244792 3.3125 0.195312C3.41927 0.143229 3.53125 0.104167 3.64844 0.078125C3.76562 0.0520833 3.88281 0.0390625 4 0.0390625C4.11719 0.0390625 4.23438 0.0520833 4.35156 0.078125C4.46875 0.104167 4.57943 0.143229 4.68359 0.195312C4.79036 0.244792 4.88932 0.307292 4.98047 0.382812C5.07161 0.458333 5.15104 0.545573 5.21875 0.644531L7.82031 4.42969C7.88021 4.51823 7.92448 4.60807 7.95312 4.69922C7.98438 4.79036 8 4.89062 8 5C8 5.13802 7.97396 5.26823 7.92188 5.39062C7.86979 5.51042 7.79948 5.61589 7.71094 5.70703C7.6224 5.79557 7.51693 5.86719 7.39453 5.92188C7.27474 5.97396 7.14714 6 7.01172 6H0.992188Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , r#MenuItem , r#AppLogic , r#Strings , r#Theme , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
