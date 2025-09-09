mod slint_generatedApp {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_13_0 = slint :: VersionCheck_1_13_0 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerMaterialPalette_42 {
         r#background : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerMaterialPalette_42 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerMaterialPalette_42 :: FIELD_OFFSETS . r#background }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294505465f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4280952877f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerMaterialPalette_42 :: FIELD_OFFSETS . r#color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_MaterialPalette_42_color_scheme = InnerMaterialPalette_42 :: FIELD_OFFSETS . r#color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () ;
                         if ! (((r#tmp_MaterialPalette_42_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Unknown))) {
                             ((((r#tmp_MaterialPalette_42_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Dark)))) as _ }
                         else {
                             ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ())) == ((sp :: r#ColorScheme :: r#Dark)) }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerMaterialPalette_42 :: FIELD_OFFSETS . r#foreground }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032031f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4293321189f64) as u32) }
                    )) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerNavItem_root_1 {
         r#root_1 : sp :: r#Rectangle , r#toucharea_2 : sp :: r#TouchArea , r#image_4 : sp :: r#ImageItem , r#text_5 : sp :: r#SimpleText , r#root_1_empty_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_icon : sp :: Property < sp :: Image > , r#root_1_is_selected : sp :: Property < bool > , r#root_1_label : sp :: Property < sp :: SharedString > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_clicked : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerNavItem_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerNavItem_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 24f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 24f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , r#size : 56f64 as _ , r#spacing : 16f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 24f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 24f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 24f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 24f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                         the_struct }
                    ]) as _ , 16f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_is_selected }
                    ) . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4278190335f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4286611584f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_is_selected }
                    ) . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4278190335f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4286611584f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             {
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
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 72f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 72f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
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
                         * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 56f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 56f64 as _ ;
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
                 0u32 => (56f64 as sp :: Coord , 72f64 as sp :: Coord , ({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (56f64 as sp :: Coord , 72f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (24f64 as sp :: Coord , 24f64 as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , 40f64 as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: r#AccessibleRole :: r#Image , 3u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
                ) . apply_pin (_self) . get ()) , _ => sp :: None , }
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
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerStateLayer_root_6 {
         r#root_6 : sp :: r#TouchArea , r#i_ripple_opacity_7 : sp :: r#Opacity , r#i_ripple_8 : sp :: r#BasicBorderRectangle , r#i_ripple_clip_9 : sp :: r#Clip , r#i_circle_10 : sp :: r#BasicBorderRectangle , r#i_focus_scope_11 : sp :: r#FocusScope , r#root_6_background : sp :: Property < slint :: Brush > , r#root_6_border_radius : sp :: Property < sp :: LogicalLength > , r#root_6_checked_background : sp :: Property < slint :: Brush > , r#root_6_focusable : sp :: Property < bool > , r#root_6_has_ripple : sp :: Property < bool > , r#root_6_height : sp :: Property < sp :: LogicalLength > , r#root_6_i_circle_10_width : sp :: Property < sp :: LogicalLength > , r#root_6_i_circle_10_x : sp :: Property < sp :: LogicalLength > , r#root_6_i_circle_10_y : sp :: Property < sp :: LogicalLength > , r#root_6_i_ripple_8_state : sp :: Property < sp :: StateInfo > , r#root_6_ripple_color : sp :: Property < slint :: Brush > , r#root_6_state : sp :: Property < i32 > , r#root_6_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerStateLayer_root_6 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerStateLayer_root_6 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             slint :: private_unstable_api :: set_animated_property_binding_for_transition (({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_width }
            ) . apply_pin (_self) , & self_rc , move | self_rc | {
                 let _self = self_rc . as_pin_ref () ;
                 (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_ripple_8_state }
                ) . apply_pin (_self) . get ()) . r#current_state as f64) , & (1f64 as f64)) {
                     ((((((((1f64) as f64) * ((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((2f64) as f64)) as f64) * ((1.4142f64) as f64))) as _ }
                 else {
                     0f64 }
                 as sp :: Coord)) as _ }
             , move | self_rc | {
                 let _self = self_rc . as_pin_ref () ;
                 {
                     let r#state = ({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_ripple_8_state }
                    ) . apply_pin (_self) . get () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#state . clone ()) . r#current_state as f64) , & (1f64 as f64)) {
                         ({
                             let mut the_struct = sp :: PropertyAnimation :: default () ;
                             the_struct . r#delay = 0f64 as _ ;
                             the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                             the_struct . r#duration = 2000f64 as _ ;
                             the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                             the_struct . r#iteration_count = 1f64 as _ ;
                             the_struct }
                        ) as _ }
                     else {
                         {
                             let mut the_struct = sp :: PropertyAnimation :: default () ;
                             the_struct . r#delay = 0f64 as _ ;
                             the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                             the_struct . r#duration = 0f64 as _ ;
                             the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                             the_struct . r#iteration_count = 0f64 as _ ;
                             the_struct }
                         }
                     , (r#state . clone ()) . r#change_time ,) }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get ()) as f64) - ((((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get ()) as f64) - ((((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_state_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_ripple_8_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) && ((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_has_ripple }
                    ) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (3f64) as _ }
                         else {
                             if ({
                                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
                             + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                                 (4f64) as _ }
                             else {
                                 0f64 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_opacity_7 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_6_state = ({
                             * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_6_state . clone () as f64) , & (1f64 as f64)) {
                             (0.12f64) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_6_state . clone () as f64) , & (2f64 as f64)) {
                                 (1f64) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_6_state . clone () as f64) , & (3f64 as f64)) {
                                     (0.08f64) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_6_state . clone () as f64) , & (4f64 as f64)) {
                                         (0.12f64) as _ }
                                     else {
                                         0f64 }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 250f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0.25f32 , 0.1f32 , 0.25f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_8 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (({
                             * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_checked_background }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         ({
                             * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_background }
                        ) . apply_pin (_self) . get () }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 250f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_8 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_border_radius }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_border_radius }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_border_radius }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_border_radius }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_border_radius }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_circle_10 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_ripple_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_circle_10 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) && ((({
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_focusable }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
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
                                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_circle_10 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_circle_10 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             {
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
                 sp :: Orientation :: Horizontal => (({
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
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , sp :: Orientation :: Vertical => (({
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
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , }
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
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((((1f64) as f64) * ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => ((((1f64) as f64) * ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => ((((1f64) as f64) * ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_i_circle_10_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
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
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , false , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMaterialButtonBase_root_12 {
         r#root_12 : sp :: r#Empty , r#state_layer_13 : InnerStateLayer_root_6 , r#root_12_colorize_icon : sp :: Property < bool > , r#root_12_height : sp :: Property < sp :: LogicalLength > , r#root_12_icon : sp :: Property < sp :: Image > , r#root_12_layout_14_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_12_layout_14_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_layout_14_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_layout_padding_left : sp :: Property < sp :: LogicalLength > , r#root_12_layout_padding_right : sp :: Property < sp :: LogicalLength > , r#root_12_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_text : sp :: Property < sp :: SharedString > , r#root_12_text_color : sp :: Property < slint :: Brush > , r#root_12_text_opacity : sp :: Property < f32 > , r#root_12_width : sp :: Property < sp :: LogicalLength > , r#root_12_x : sp :: Property < sp :: LogicalLength > , repeater0 : sp :: Conditional < InnerComponent__opacity_15 > , repeater1 : sp :: Conditional < InnerComponent__opacity_18 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMaterialButtonBase_root_12 {
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
                     ({
                         let r#tmp_root_12_icon = ({
                             * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_icon }
                        ) . apply_pin (_self) . get () ;
                         (((((r#tmp_root_12_icon . clone () . size ()) . r#width) as f64) > ((0f64) as f64))) && (((((r#tmp_root_12_icon . clone () . size ()) . r#height) as f64) > ((0f64) as f64))) }
                    ) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text }
                    ) . apply_pin (_self) . get ())) != ((sp :: SharedString :: from (""))))) as _ }
                 }
            ) ;
             InnerStateLayer_root_6 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#state_layer_13 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 1u32 - 1 , tree_index_of_first_child + 4u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_18 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = ({
                                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_left }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#end = ({
                                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_right }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_18 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = ({
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_left }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct . r#end = ({
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_right }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_18 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
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
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
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
                             the_struct . r#min = (({
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_0 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#max as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#min as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#preferred as _ ;
                         the_struct . r#stretch = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#stretch as _ ;
                         the_struct }
                    )))) + ((({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
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
                             the_struct . r#min = (({
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_1 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#max as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#min as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#preferred as _ ;
                         the_struct . r#stretch = (((({
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
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )))) . r#stretch as _ ;
                         the_struct }
                    )))) + ((({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032031f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4293321189f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_checked_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4284960932f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4291869951f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_focusable }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_has_ripple }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1f64) as f64) * ((({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_ripple_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3439329279f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4278190080f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1f64) as f64) * ((({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_focusable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_has_ripple }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerStateLayer_root_6 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#state_layer_13 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_18 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (({
                             * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (({
                             * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as _ ;
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
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_18 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_15 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_18 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
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
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((((1f64) as f64) * ((({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 ..= 8u32 => return {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . item_geometry (index - 4u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . accessible_role (0) , 4u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . accessible_role (index - 4u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (4u32 ..= 8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . accessible_string_property (index - 4u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (4u32 ..= 8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . accessibility_action (index - 4u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 4u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 4u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#state_layer_13 }
                 . apply_pin (_self) . item_element_infos (index - 4u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 . apply_pin (_self) . r#fn_clear_focus ()) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 . apply_pin (_self) . r#fn_focus ()) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_15 {
         r#_opacity_15 : sp :: r#Opacity , r#image_16 : sp :: r#ImageItem , r#_opacity_15_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_15_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_15 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_15 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | sp :: Property :: link_two_way (({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#_opacity_15_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#_opacity_15_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#_opacity_15 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text_opacity) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_colorize_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () {
                         ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text_color) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
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
                     let r#layout_info = ({
                         * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#_opacity_15_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 24f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 24f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#_opacity_15_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
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
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 24f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 24f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
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
     impl InnerComponent__opacity_15 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 > ;
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
                 is_accessible : true , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_15 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#_opacity_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_15 :: FIELD_OFFSETS . r#image_16 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_15) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_15 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent__opacity_15 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_15 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_15 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     impl sp :: RepeatedItemTree for InnerComponent__opacity_15 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_15 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_18 {
         r#_opacity_18 : sp :: r#Opacity , r#text_19 : sp :: r#SimpleText , r#_opacity_18_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_18_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_18 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_18 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#_opacity_18_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#_opacity_18_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#_opacity_18 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text_opacity) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text_color) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 250f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0.25f32 , 0.1f32 , 0.25f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (((500f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
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
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#_opacity_18_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#_opacity_18_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
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
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
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
     impl InnerComponent__opacity_18 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMaterialButtonBase_root_12 > ;
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
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_18 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#_opacity_18 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_18 :: FIELD_OFFSETS . r#text_19 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_18) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_18 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent__opacity_18 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_18 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_18 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     impl sp :: RepeatedItemTree for InnerComponent__opacity_18 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_18 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_21 {
         r#root_21 : sp :: r#Empty , r#background_opacity_22 : sp :: r#Opacity , r#background_shadow_23 : sp :: r#BoxShadow , r#background_24 : sp :: r#BasicBorderRectangle , r#base_26 : InnerMaterialButtonBase_root_12 , r#root_21_checked : sp :: Property < bool > , r#root_21_height : sp :: Property < sp :: LogicalLength > , r#root_21_layout_25_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_21_layout_25_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_layout_25_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_21_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_21_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_21_state : sp :: Property < i32 > , r#root_21_width : sp :: Property < sp :: LogicalLength > , r#root_21_x : sp :: Property < sp :: LogicalLength > , r#root_21_y : sp :: Property < sp :: LogicalLength > , r#root_21_accessible_action_default : sp :: Callback < () , () > , r#root_21_clicked : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_21 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_21 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMaterialButtonBase_root_12 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_26 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 5u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_accessible_action_default }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                             + {
                                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                             + {
                                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                                 + {
                                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (({
                                         InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                                     + {
                                         * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                             + {
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (({
                                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                                 + {
                                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                             + {
                                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (({
                                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                                 + {
                                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_14_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
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
                             the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_2 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_2 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_2 . clone ()) . r#stretch as _ ;
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
                    )))) + ((({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((({
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
                             the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_3 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_3 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_3 . clone ()) . r#stretch as _ ;
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
                    )))) + ((({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                     + {
                         InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                     + {
                         * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_checked }
                        ) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_opacity_22 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (0.12f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded ((0f64) as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_24 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4280032031f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293321189f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4293451512f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4283057240f64) as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6_border_radius }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     ({
                                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_checked }
                                    ) . apply_pin (_self) . set ((! ({
                                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_checked }
                                    ) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             ({
                                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_clicked }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_left }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_right }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_21_state = ({
                             * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_21_state . clone () as f64) , & (1f64 as f64)) {
                             (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((4280162603f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((4293451512f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_21_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4281802355f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if ! InnerMaterialPalette_42 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((4280162603f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4293451512f64) as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text_opacity }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (0.38f64) as _ }
                     else {
                         1f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_colorize_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_left }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_layout_padding_right }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMaterialButtonBase_root_12 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_26 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_26 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
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
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_26 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_26 }
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
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((((1f64) as f64) * ((({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => ((((1f64) as f64) * ((({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => ((((1f64) as f64) * ((({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , (((1f64) as f64) * ((({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 ..= 12u32 => return {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . item_geometry (index - 5u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . accessible_role (0) , 5u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . accessible_role (index - 5u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text }
                ) . apply_pin (_self) . get ()) , (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (5u32 ..= 12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . accessible_string_property (index - 5u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (5u32 ..= 12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . accessibility_action (index - 5u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 5u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 5u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_26 }
                 . apply_pin (_self) . item_element_infos (index - 5u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerApp {
         r#root_27 : sp :: r#WindowItem , r#rectangle_29 : sp :: r#Rectangle , r#text_30 : sp :: r#SimpleText , r#rectangle_31 : sp :: r#Rectangle , r#_visibility_36 : sp :: r#Clip , r#rectangle_37 : sp :: r#BasicBorderRectangle , r#text_39 : sp :: r#SimpleText , r#navitem_33 : InnerNavItem_root_1 , r#navitem_34 : InnerNavItem_root_1 , r#navitem_35 : InnerNavItem_root_1 , r#button_40 : InnerButton_root_21 , r#root_27_dialog_text : sp :: Property < sp :: SharedString > , r#root_27_empty_28_height : sp :: Property < sp :: LogicalLength > , r#root_27_empty_28_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_27_empty_28_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_27_empty_28_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_27_empty_28_width : sp :: Property < sp :: LogicalLength > , r#root_27_empty_32_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_27_empty_32_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_27_empty_32_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_27_empty_38_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_27_empty_38_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_27_empty_38_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_27_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_27_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_27_rectangle_29_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_27_rectangle_29_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_27_rectangle_31_width : sp :: Property < sp :: LogicalLength > , r#root_27_rectangle_37_y : sp :: Property < sp :: LogicalLength > , r#root_27_selected_item : sp :: Property < sp :: SharedString > , r#root_27_show_dialog : sp :: Property < bool > , r#root_27_text_30_min_height : sp :: Property < sp :: LogicalLength > , r#root_27_text_30_min_width : sp :: Property < sp :: LogicalLength > , r#root_27_text_30_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_27_text_30_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_27_text_30_x : sp :: Property < sp :: LogicalLength > , r#root_27_text_30_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerApp >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerApp {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerNavItem_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#navitem_33 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 8u32 - 1) ;
             InnerNavItem_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#navitem_34 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 11u32 - 1) ;
             InnerNavItem_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#navitem_35 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 7u32 - 1 , tree_index_of_first_child + 14u32 - 1) ;
             InnerButton_root_21 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_40 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 19u32 - 1 , tree_index_of_first_child + 20u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerMaterialPalette_42 :: FIELD_OFFSETS . r#background . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Roboto")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_29_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (((({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((56f64) as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (((({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((56f64) as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
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
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_height }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_height }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 16f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_29_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
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
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_29_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (((({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((56f64) as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (((({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((56f64) as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
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
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 16f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
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
                                     InnerApp :: FIELD_OFFSETS . r#navitem_33 }
                                 + {
                                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 72f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 72f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
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
                                     InnerApp :: FIELD_OFFSETS . r#navitem_34 }
                                 + {
                                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 72f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 72f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
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
                                     InnerApp :: FIELD_OFFSETS . r#navitem_35 }
                                 + {
                                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 72f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 72f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_31_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layoutinfo_h }
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
                                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
                             + {
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 72f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 72f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
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
                                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
                             + {
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 72f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 72f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
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
                                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
                             + {
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 72f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 72f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layoutinfo_v }
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
                                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
                             + {
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 56f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 56f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
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
                                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
                             + {
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 56f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 56f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
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
                                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
                             + {
                                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 56f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 56f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 18u32 - 1)) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                                 + {
                                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                                         InnerApp :: FIELD_OFFSETS . r#button_40 }
                                     + {
                                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 16f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerApp :: FIELD_OFFSETS . r#text_39 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 18u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerApp :: FIELD_OFFSETS . r#button_40 }
                             + {
                                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                                 + {
                                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerApp :: FIELD_OFFSETS . r#text_39 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 18u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerApp :: FIELD_OFFSETS . r#button_40 }
                             + {
                                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (40f64 as sp :: Coord) . max ((({
                                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                                 + {
                                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_layout_25_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 16f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())))) + ((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())))) + ((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_29_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_29_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_31_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_37_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Home")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((16f64) as f64)) as f64) - ((16f64) as f64)) as f64) - ((({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                     + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((56f64) as f64)) as f64) - ((({
                         * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                     + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Bottom Navigation App")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_29 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294967295f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerMaterialPalette_42 :: FIELD_OFFSETS . r#foreground . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((sp :: SharedString :: from ("Content for "))) + ((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                    ) . apply_pin (_self) . get ()) . as_str ()))) + ((sp :: SharedString :: from (" screen")) . as_str ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_31 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((2296038082f64) as u32))) as slint :: Brush }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_33 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                            ) . apply_pin (_self) . set (sp :: SharedString :: from ("Home") as _) ;
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
                            ) . apply_pin (_self) . set (sp :: SharedString :: from ("Home Selected") as _) ;
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
                            ) . apply_pin (_self) . set (true as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"png"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_33 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_is_selected }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                    ) . apply_pin (_self) . get ())) == ((sp :: SharedString :: from ("Home"))))) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Home")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_33 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_34 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                            ) . apply_pin (_self) . set (sp :: SharedString :: from ("Search") as _) ;
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
                            ) . apply_pin (_self) . set (sp :: SharedString :: from ("Search Selected") as _) ;
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
                            ) . apply_pin (_self) . set (true as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"png"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_34 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_is_selected }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                    ) . apply_pin (_self) . get ())) == ((sp :: SharedString :: from ("Search"))))) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Search")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_34 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_35 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                            ) . apply_pin (_self) . set (sp :: SharedString :: from ("Profile") as _) ;
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
                            ) . apply_pin (_self) . set (sp :: SharedString :: from ("Profile Selected") as _) ;
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
                            ) . apply_pin (_self) . set (true as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"png"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_35 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_is_selected }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
                    ) . apply_pin (_self) . get ())) == ((sp :: SharedString :: from ("Profile"))))) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Profile")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#navitem_35 }
                 + {
                     * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294967295f64) as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4286611584f64) as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerMaterialPalette_42 :: FIELD_OFFSETS . r#foreground . apply_pin (_self . globals . get () . unwrap () . global_MaterialPalette_42 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (268f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("OK")) as sp :: SharedString }
            ) ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (268f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_29 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_31 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_label }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerNavItem_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#navitem_33 }
             . apply_pin (x)) ,) ;
             InnerNavItem_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#navitem_34 }
             . apply_pin (x)) ,) ;
             InnerNavItem_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#navitem_35 }
             . apply_pin (x)) ,) ;
             InnerButton_root_21 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_40 }
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
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_40 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_40 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_40 }
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
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((((({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64) - ((56f64) as f64)) as sp :: Coord , (((((({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((16f64) as f64)) as f64) - ((16f64) as f64)) as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_28_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_text_30_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (56f64 as sp :: Coord , 72f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 16f64 as sp :: Coord ,) , 6u32 => (56f64 as sp :: Coord , 72f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 16f64 as sp :: Coord ,) , 7u32 => (56f64 as sp :: Coord , 72f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_32_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 16f64 as sp :: Coord ,) , 17u32 => (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , 300f64 as sp :: Coord , (((((({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((300f64) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_rectangle_37_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 18u32 => (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 268f64 as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 19u32 => (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , 268f64 as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 8u32 ..= 10u32 => return {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . item_geometry (index - 8u32 + 1) , 11u32 ..= 13u32 => return {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . item_geometry (index - 11u32 + 1) , 14u32 ..= 16u32 => return {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . item_geometry (index - 14u32 + 1) , 20u32 ..= 31u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . item_geometry (index - 20u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: r#AccessibleRole :: r#Text , 18u32 => sp :: r#AccessibleRole :: r#Text , 19u32 => sp :: r#AccessibleRole :: r#Button , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . accessible_role (0) , 8u32 ..= 10u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . accessible_role (index - 8u32 + 1) , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . accessible_role (0) , 11u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . accessible_role (index - 11u32 + 1) , 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . accessible_role (0) , 14u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . accessible_role (index - 14u32 + 1) , 19u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_role (0) , 20u32 ..= 31u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_role (index - 20u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerApp :: FIELD_OFFSETS . r#text_30 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (18u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
                ) . apply_pin (_self) . get ()) , (19u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (19u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (19u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                 + {
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
                 + {
                     * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (19u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerApp :: FIELD_OFFSETS . r#button_40 }
                 + {
                     InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
                 + {
                     * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12_text }
                ) . apply_pin (_self) . get ()) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (8u32 ..= 10u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . accessible_string_property (index - 8u32 + 1 , what) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (11u32 ..= 13u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . accessible_string_property (index - 11u32 + 1 , what) , (7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (14u32 ..= 16u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . accessible_string_property (index - 14u32 + 1 , what) , (19u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (20u32 ..= 31u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_string_property (index - 20u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (19u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerApp :: FIELD_OFFSETS . r#button_40 }
                     + {
                         * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (8u32 ..= 10u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . accessibility_action (index - 8u32 + 1 , action) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (11u32 ..= 13u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . accessibility_action (index - 11u32 + 1 , action) , (7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (14u32 ..= 16u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . accessibility_action (index - 14u32 + 1 , action) , (19u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (20u32 ..= 31u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessibility_action (index - 20u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 19u32 => sp :: SupportedAccessibilityAction :: r#Default , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 8u32 ..= 10u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 8u32 + 1) , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 11u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 11u32 + 1) , 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 14u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 14u32 + 1) , 19u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 20u32 ..= 31u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 20u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 8u32 ..= 10u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_33 }
                 . apply_pin (_self) . item_element_infos (index - 8u32 + 1) , 11u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_34 }
                 . apply_pin (_self) . item_element_infos (index - 11u32 + 1) , 14u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#navitem_35 }
                 . apply_pin (_self) . item_element_infos (index - 14u32 + 1) , 20u32 ..= 31u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . item_element_infos (index - 20u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerApp {
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
             32usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 8u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 11u32 , parent_index : 2u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 14u32 , parent_index : 2u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 11u32 , parent_index : 5u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 11u32 , parent_index : 5u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 11u32 , parent_index : 5u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 6u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 14u32 , parent_index : 6u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 14u32 , parent_index : 6u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 7u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 17u32 , parent_index : 7u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 17u32 , parent_index : 7u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 18u32 , parent_index : 3u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 20u32 , parent_index : 17u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 20u32 , parent_index : 17u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 22u32 , parent_index : 19u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 24u32 , parent_index : 19u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 20u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 20u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 27u32 , parent_index : 21u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 29u32 , parent_index : 24u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 32u32 , parent_index : 24u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 30u32 , parent_index : 27u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 31u32 , parent_index : 29u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 32u32 , parent_index : 30u32 , item_array_index : 29u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerApp , sp :: ItemVTable , sp :: AllowPin > ;
             30usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27 }
            ) , sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_29 }
            ) , sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#_visibility_36 }
            ) , sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_30 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_33 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_34 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#toucharea_2 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#image_4 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#navitem_35 }
             + {
                 * & InnerNavItem_root_1 :: FIELD_OFFSETS . r#text_5 }
            ) , sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_37 }
            ) , sp :: VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#text_39 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#root_21 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_opacity_22 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 * & InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#root_12 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_shadow_23 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_21 :: FIELD_OFFSETS . r#background_24 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#root_6 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_opacity_7 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_focus_scope_11 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_8 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_ripple_clip_9 }
            ) , sp :: VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#button_40 }
             + {
                 InnerButton_root_21 :: FIELD_OFFSETS . r#base_26 }
             + {
                 InnerMaterialButtonBase_root_12 :: FIELD_OFFSETS . r#state_layer_13 }
             + {
                 * & InnerStateLayer_root_6 :: FIELD_OFFSETS . r#i_circle_10 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerApp) ;
         }
     ;
     impl sp :: PinnedDrop for InnerApp {
         fn drop (self : :: core :: pin :: Pin < & mut InnerApp >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerApp {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerApp > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     pub struct r#App (sp :: VRc < sp :: ItemTreeVTable , InnerApp >) ;
     impl r#App {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerApp :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerApp :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_dialog_text (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_dialog_text (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_dialog_text }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_selected_item (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_selected_item (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_selected_item }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_show_dialog (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_show_dialog (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_27_show_dialog }
            ) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#App > for sp :: VRc < sp :: ItemTreeVTable , InnerApp > {
         fn from (value : r#App) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#App {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerApp > ;
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
         global_MaterialPalette_42 : :: core :: pin :: Pin < sp :: Rc < InnerMaterialPalette_42 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_MaterialPalette_42 : InnerMaterialPalette_42 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_MaterialPalette_42 . clone () . init (& _self) ;
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
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = :: core :: include_bytes ! ("E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-bottom-nav\\android\\res\\mipmap\\home.png") ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-bottom-nav\\android\\res\\mipmap\\profile.png") ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = :: core :: include_bytes ! ("E:\\ProgrammingLang\\Rust\\rust-projects\\AndroidRust\\slint-bottom-nav\\android\\res\\mipmap\\search.png") ;
     }
 # [allow (unused_imports)] pub use slint_generatedApp :: {
     r#App , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
