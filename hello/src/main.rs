use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::{Renderer, Affine};
use gate::gate_header;

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use crate::asset_id::{AssetId, SpriteId, MusicId, SoundId};

gate_header!();

struct HelloGame {
    posx: usize,
    posy: usize,
}

impl App<AssetId> for HelloGame {
    fn advance(&mut self, seconds: f64, _ctx: &mut AppContext<AssetId>) {
    }
    fn key_down(&mut self, key: KeyCode, ctx: &mut AppContext<AssetId>) {
        match key {
            KeyCode::Left=>{if self.posx > 0 {self.posx-=1;}},
            KeyCode::Right=>{if self.posx < 4 {self.posx+=1;}},
            KeyCode::Down=>{if self.posy > 0 {self.posy-=1;}},
            KeyCode::Up=>{if self.posy < 4 {self.posy+=1;}},
            _=>{}
        };
    }
    fn render(&mut self, renderer: &mut Renderer<AssetId>, ctx: &AppContext<AssetId>) {
        let (app_width, app_height) = ctx.dims();
        let mut renderer = renderer.flash_mode();
        for x in 0..((app_width / 16.).ceil() as usize) {
            for y in 0..((app_height / 16.).ceil() as usize) {
                let affine = Affine::translate(8. + x as f64 * 16., 8. + y as f64 * 16.);
                let tile = SpriteId::wall ;
                renderer.draw(&affine, tile);
            }
        }
        // let base = Affine::translate(0.5 * app_width, 0.5 * app_height - 5.).pre_scale(0.5);
        let base = Affine::translate(0., 0.).pre_scale(0.5);
        // let xx = (app_width / 16.).ceil() as usize;
        renderer.draw(&base.pre_translate(8. + (self.posx  )as f64 * 16., 8.+(self.posy )as f64 * 16.), SpriteId::player);
    }
}

fn main() {
    let info = AppInfo::with_max_dims(86., 48.)
                       .min_dims(64., 44.)
                       .tile_width(16)
                       .title("Hello");
    gate::run(info, |ctx| {
        ctx.audio.loop_music(MusicId::TwintrisThosenine);
        HelloGame {posx:0, posy:0 }});
}
