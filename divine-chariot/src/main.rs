#![allow(dead_code)]
#![allow(redundant_semicolons)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]

mod b64;
mod core_systems;
mod entity;
mod grid;
mod handle;
mod hash;
mod location;
mod map;
mod paths;
mod perlin;
mod point2d;
mod priority_queue;
mod rect2d;
mod resource;
mod rng;
mod time_manager;
mod value;
mod window;

use bear_lib_terminal::{geometry,terminal};

use crate::core_systems::*;
use crate::entity::{EntityId, EntityManagerHandle};
use crate::grid::{Grid};
use crate::handle::{Handle};
use crate::perlin::{Perlin};
use crate::point2d::*;
use crate::rect2d::*;
use crate::rng::{Rnd, Generator, Rng, Sampler};
use crate::time_manager::{TimeManager};
use crate::window::{WindowHandle};

////////////////////////////////////////////////////////////////////////////////

pub struct FontConfig(pub String);
impl terminal::config::ConfigPart for FontConfig {
  fn to_config_str(&self) -> String { self.0.clone() }
}

pub fn window_write_str(window:&WindowHandle<Glyph>, s:&str, p:Point2d, fg:Color, bg:Color) {
  let mut p = p;
  for ch in s.chars() {
    window.set(p, Glyph::new(ch, fg, bg));
    p.x += 1;
  }
}

////////////////////////////////////////////////////////////////////////////////

pub struct MapViewWindow {
  window : WindowHandle<Glyph>,
  span : Rect2d,
  eid : EntityId,
  perlin : Perlin,
}

impl MapViewWindow {
  pub fn new(parent:&WindowHandle<Glyph>, offset:Point2d, span:Rect2d, eid:EntityId) -> Self {
    let window = parent.new_child(format!("MapView[{}]",eid), offset, span);
    let perlin = Perlin::new();
    let mut res = MapViewWindow { window, span, eid, perlin, };
    res.update(0);
    res
  }

  pub fn update(&mut self, frame:usize) {
    if frame % 5 == 0 {
      for mwp in self.span {
        let z = frame as f64/120.0;
        let v = self.perlin.octave_perlin(
            ((self.span.bl.x + mwp.x) as f64)/self.span.size().x as f64*3.0,
            ((self.span.bl.y + mwp.y) as f64)/self.span.size().y as f64*3.0,
            z, 2, 0.5).powi(2);
        let v = (v * 256.0).floor().min(255.0).floor() as u8;
        let v = (v/32)*32;
        self.window.set(mwp, Glyph::new(' ',Color::jade(),Color::rgb(v,v,v)));
      }
    }
  }
}

impl Drop for MapViewWindow {
  fn drop(&mut self) {
    self.window.demolish();
  }
}

////////////////////////////////////////////////////////////////////////////////

pub fn main() {
  let terminal_span = Rect2d::new(Point2d::new(0,0), Point2d::new(100,40));
  terminal::open("Test", terminal_span.tr.x as u32, terminal_span.tr.y as u32);
  terminal::set(terminal::config::Window::empty()
    .title("Divine-Chariot".to_string())
    .size(geometry::Size::new(terminal_span.tr.x, terminal_span.tr.y))
    .cellsize(terminal::config::Cellsize::Auto));
  terminal::set(vec![terminal::config::InputFilter::Group{group: terminal::config::InputFilterGroup::Keyboard, both: false}]);
  //"font: default;"
  terminal::set(FontConfig("font: ./DejaVuSansMono.ttf, size=14;".into()));

  println!("Hello, world!");

  let entity_manager = EntityManagerHandle::new();
  let player_id = entity_manager.new_id();

  //let z = terminal_span.size().y - 2;
  //let map_window_span = Rect2d::new(Point2d::new(-z/2,-z/2),Point2d::new(z/2,z/2));
  let map_window_span = Rect2d::new(Point2d::new(-40,-15),Point2d::new(40,15));
  let map_window_offset = Point2d::new(1,1);
  let map_window_size = map_window_span.size();

  let base_window : WindowHandle<Glyph> = WindowHandle::new("Terminal".into(), terminal_span);
  let mut mapview_window = MapViewWindow::new( &base_window, Point2d::new(1,1), map_window_span, player_id);
  base_window.dump_tree();
  {
    let mdd = ((terminal_span.size().x*terminal_span.size().x + terminal_span.size().y*terminal_span.size().y) as f64).sqrt();
    let col = |x,y| {
        let dx = x - terminal_span.bl.x;
        let dy = y - terminal_span.bl.y;
        let d = (((dx*dx + dy*dy) as f64).sqrt() / mdd);
        Color::mistyrose().lint(&Color::rose_garnet(), d)
        //Color::rose_garnet().lint(&Color::cadmium_blue(), d)
        //Color::iron().lint(&Color::jade(), d)
        //Color::english_lavender().lint(&Color::jade(), d)
      };
    for p in terminal_span.boundary_iter() {
      base_window.set(p, Glyph::new('█',col(p.x,p.y),Color::black()));
    }
    for y in terminal_span.bl.y .. terminal_span.tr.y {
      //let x = base_window.span().bl.x + map_window_offset.x + map_window_size.x;
      let x = base_window.span().bl.x + 0 + 0;
      base_window.set(Point2d::new(x,y), Glyph::new('█',col(x,y),Color::black()));
    }
  }

  let mut quit = false;
  let mut time_manager = TimeManager::new(61);
  time_manager.unpause();
  let mut n = 0;
  while !quit {
    //game.step();
    time_manager.tick();

    /*
    let mut rng = Rng::from_seed([3,13,75,34857]);
    for termp in terminal_span {
      //let glyph = base_window.data(termp).unwrap();
      //terminal::set_background(glyph.bg.blt());
      //terminal::set_foreground(glyph.fg.blt());
      //terminal::put_xy(termp.x, termp.y, glyph.ch);
      let x = ((termp.x * 256) / terminal_span.tr.x) as u8;
      let y = ((termp.y * 256) / terminal_span.tr.y) as u8;
      //terminal::set_foreground(Color::black().blt());
      //terminal::set_background(Color::rgb(x, y, 255-(x/2+y/2)).blt());
      let f_r : u8 = rng.gen(); let f_g : u8 = rng.gen(); let f_b : u8 = rng.gen();
      let b_r : u8 = rng.gen(); let b_g : u8 = rng.gen(); let b_b : u8 = rng.gen();
      //terminal::set_foreground(Color::rgb(f_r,f_g,f_b).blt());
      terminal::set_foreground(Color::black().blt());
      terminal::set_background(Color::rgb(f_r/8,x/8,y/8).blt());
      terminal::put_xy(termp.x, termp.y, ['.',',','X','O','*','#'][(rng.next_u32()%6) as usize]);
    }
    */
    mapview_window.update(time_manager.frame_count());
    base_window.update_data();
    for termp in terminal_span {
      let glyph = base_window.data(termp).unwrap();
      terminal::set_background(glyph.bg.blt());
      terminal::set_foreground(glyph.fg.blt());
      terminal::put_xy(termp.x, termp.y, glyph.ch);
    }

    terminal::set_background(Color::white().scale(0.1).blt());
    terminal::set_foreground(Color::mistyrose().blt());
    terminal::print_xy(80, 18, &format!("fps:{:.1}/{}", time_manager.average_fps(), time_manager.target_fps()));
    //terminal::set_foreground(Color::azure_mist().blt());
    //terminal::print_xy(80, 17, &format!("t:{}", game.now()));
    //terminal::set_foreground(Color::parakeet_blue().blt());
    //terminal::print_xy(80, 16, &format!("{:?}", game.state()));
    terminal::set_foreground(Color::mint().blt());
    terminal::print_xy(80, 15, "ḫaiāṭum");
    terminal::set_foreground(Color::gold().blt());
    terminal::print_xy(80, 14, "ḫurāṣum");
    terminal::set_foreground(Color::iron().blt());
    terminal::print_xy(80, 13, "kaspum");

    terminal::refresh();

    if terminal::has_input() {
      let ev = terminal::wait_event();
      match ev {
        Some(terminal::Event::KeyPressed{key:k,ctrl,shift,alt}) => {
          match k {
            terminal::KeyCode::Escape => { quit = true; }
            terminal::KeyCode::Q => { quit = true; }
            _ => {}
          }
        }
        Some(terminal::Event::Close) => { quit = true; }
        _ => {}
      }
    }
  }
  terminal::close();

  println!("Goodbye, world!");

  println!();
  println!("ḫurāṣum = gold");
  println!("kaspum : [Nature → Metals]  1) silver ; 2) (currency) : price / money ;");
  println!("ṣarpum : [Nature → Metals]  silver");
  println!("ḫaiāṭum = a spy , a surveyor , an inspector , a watchman , a warden");
  println!("\u{cf88}  魔界");
}
