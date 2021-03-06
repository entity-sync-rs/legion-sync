// use std::{thread, time::Duration};
//
// use legion::prelude::{any, IntoQuery, Read, Resources, Schedulable, Schedule, SystemBuilder, Universe, passthrough};
//
// use legion_sync::{
//     components::UidComponent,
//     resources::{EventResource, RegisteredComponentsResource},
//     systems::track_modifications_system,
//     tracking::*,
// };
// use net_sync::uid::Uid;
//
// #[sync]
// #[derive(Debug)]
// pub struct Position {
//     pub x: u16,
//     pub y: u16,
// }
//
// impl Position {
//     pub fn set(&mut self, pos: (u16, u16)) {
//         self.x = pos.0;
//         self.y = pos.1;
//     }
// }
//
// impl Default for Position {
//     fn default() -> Self {
//         Position { x: 0, y: 0 }
//     }
// }
//
// fn main() {
//     let world = Universe::new();
//     let mut world = world.create_world();
//
//     let mut resources = Resources::default();
//     resources.insert(PostOfficeResource::new());
//     resources.insert(EventResource::new(&mut world, passthrough()));
//     resources.insert(RegisteredComponentsResource::new());
//
//     world.insert(
//         (),
//         vec![(Position { x: 1, y: 1 }, UidComponent::new(Uid(1)))],
//     );
//
//     let mut scheduler = Schedule::builder()
//         .add_system(track_modifications_system())
//         .add_system(make_modification_system())
//         .add_system(watch_modifications_system())
//         .build();
//
//     loop {
//         scheduler.execute(&mut world, &mut resources);
//
//         thread::sleep(Duration::from_millis(10));
//     }
// }
//
// pub fn make_modification_system() -> Box<dyn Schedulable> {
//     SystemBuilder::new("move player")
//         .read_resource::<EventResource>()
//         .with_query(<(legion::prelude::Write<Position>, Read<UidComponent>)>::query())
//         .build(|_, mut world, resource, query| {
//             for (mut pos, identifier) in query.iter_mut(&mut world) {
//                 let mut pos = pos.track(resource.notifier(), identifier.uid());
//                 let new_pos = (pos.x + 1, pos.x + 1);
//                 pos.set(new_pos);
//             }
//         })
// }
//
// pub fn watch_modifications_system() -> Box<dyn Schedulable> {
//     SystemBuilder::new("read_received_system")
//         .write_resource::<PostOfficeResource>()
//         .build(|_, _, sent_buffer, _| {
//             for message in sent_buffer.drain_messages(|_| true).into_iter() {
//                 print!("urgency: {:?} \t| ", message.urgency());
//                 println!("event: {:?}", message.event());
//
//                 // - sent data over to other endpoint (see tcp_sent_system)
//                 // - apply changed data to struct (see `Apply`)
//             }
//         })
// }

fn main() {}
