struct PpuCtrl {
    pub nmi_enable:         bool,
    pub master_slave:       bool,
    pub sprite_height:      bool,
    pub bgnd_tile_select:   bool,
    pub sprite_tile_select: bool,
    pub increment_mode:     bool,
    pub nametable_select:   u8,
}
