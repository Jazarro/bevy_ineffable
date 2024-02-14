// use std::time::Duration;
//
// use bevy::log::error;
// use bevy::prelude::TimerMode;
// use bevy::time::{Stopwatch, Timer};
// use bevy::utils::default;
//
// use crate::bindings::{ InputBinding};
// use crate::input_action::InputAction;
// use crate::phantom::{Charged, IAWrp};
// use crate::prelude::Ineffable;
// use crate::processed::bound_action::BoundAction;
// use crate::processed::processor::Helper;
// use crate::processed::stateful::binary_input;
// use crate::processed::stateful::binary_input::StatefulBinaryInput;
// use crate::processed::updating::InputSources;
// use crate::reporting::{ActionLocation, InputConfigProblem, InputConfigReport};
// use crate::resources::meta_data::IneffableMetaItem;
//
// /// Used for the preset charged variant. This is the minimum charge duration that can be set.
// /// This is based on roughly how quickly a human could tap and let go of a key in the real world.
// /// Setting this as a floor prevents strange glitches caused by mistaken assumptions about the charge length.
// pub const MINIMUM_CHARGE_DURATION_MILLIS: u64 = 50;
//
// #[derive(Debug, Default, Clone)]
// pub(crate) struct StatefulChargedBinding {
//     bindings: Vec<StatefulChargedBindingVariant>,
//     time_active: Stopwatch,
//     toggled_on: bool,
//     pub(crate) charging: bool,
//     pub(crate) just_released: bool,
// }
//
// #[derive(Debug, Clone)]
// pub(crate) enum StatefulChargedBindingVariant {
//     Dummy,
//     Held(StatefulBinaryInput),
//     Toggle(StatefulBinaryInput),
//     Preset(Timer, StatefulBinaryInput),
// }
//
// pub(crate) fn bound_action<I: InputAction>(
//     ineffable: &Ineffable,
//     input_action: IAWrp<I, Charged>,
// ) -> Option<&StatefulChargedBinding> {
//     ineffable
//         .groups.get(I::group_id())?.get(input_action.0.index())
//         .and_then(|bound_action| {
//             if let BoundAction::Charged(binding) = bound_action {
//                 Some(binding)
//             } else {
//                 error!("Please use the ineff!() macro for a compile-time guarantee that you're using the correct InputKind.");
//                 None
//             }
//         })
// }
//
// pub(crate) fn collect<'a>(
//     out: &mut Helper<'a>,
//     meta: &'a IneffableMetaItem,
//     binding: &ChargedBinding,
// ) {
//     match binding {
//         ChargedBinding::Dummy => {}
//         ChargedBinding::Hold(input)
//         | ChargedBinding::Toggle(input)
//         | ChargedBinding::Preset(_, input) => {
//             out.push(meta, input.clone());
//         }
//     }
// }
//
// pub(crate) fn check_for_problems(
//     charged: &ChargedBinding,
//     report: &mut InputConfigReport,
//     loc: &ActionLocation,
// ) {
//     match charged {
//         ChargedBinding::Dummy => (),
//         ChargedBinding::Hold(input)
//         | ChargedBinding::Toggle(input)
//         | ChargedBinding::Preset(_, input) => {
//             if input.is_empty() {
//                 report.warning(InputConfigProblem::ConvolutedDummy {
//                     loc: loc.clone(),
//                     is_now: format!("{charged:?}"),
//                 });
//             }
//             binary_input::check_for_problems(input, report, loc);
//         }
//     }
//     if let ChargedBinding::Preset(millis, _) = charged {
//         if *millis < MINIMUM_CHARGE_DURATION_MILLIS {
//             report.warning(InputConfigProblem::ChargedPresetDurationTooShort {
//                 loc: loc.clone(),
//                 actual_millis: *millis,
//             });
//         }
//     }
// }
//
// impl StatefulChargedBinding {
//     pub fn time_active(&self) -> Duration {
//         self.time_active.elapsed()
//     }
//     pub fn charging_duration(&self) -> Option<Duration> {
//         if self.time_active.elapsed().is_zero() {
//             None
//         } else {
//             Some(self.time_active.elapsed())
//         }
//     }
//
//     pub(crate) fn new(data: &[InputBinding], helper: &Helper<'_>) -> StatefulChargedBinding {
//         let stateful_bindings = data
//             .iter()
//             .filter_map(|binding| {
//                 if let InputBinding::Charged(charged) = binding {
//                     Some(charged)
//                 } else {
//                     None
//                 }
//             })
//             .map(|charged| match charged {
//                 ChargedBinding::Dummy => StatefulChargedBindingVariant::Dummy,
//                 ChargedBinding::Hold(input) => {
//                     StatefulChargedBindingVariant::Held(StatefulBinaryInput::new(input, helper))
//                 }
//                 ChargedBinding::Toggle(input) => {
//                     StatefulChargedBindingVariant::Toggle(StatefulBinaryInput::new(input, helper))
//                 }
//                 ChargedBinding::Preset(millis, input) => {
//                     let mut timer = Timer::new(
//                         Duration::from_millis(MINIMUM_CHARGE_DURATION_MILLIS.max(*millis)),
//                         TimerMode::Once,
//                     );
//                     timer.pause();
//                     StatefulChargedBindingVariant::Preset(
//                         timer,
//                         StatefulBinaryInput::new(input, helper),
//                     )
//                 }
//             })
//             .collect();
//         StatefulChargedBinding {
//             bindings: stateful_bindings,
//             ..default()
//         }
//     }
//
//     pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
//         if self.just_released {
//             // If it released last tick, then reset the timer.
//             self.time_active.reset();
//         }
//         let (mut held, mut just_released, toggle) = self.bindings.iter_mut().fold(
//             (false, false, false),
//             |(held, just_released, toggle), variant| match variant {
//                 StatefulChargedBindingVariant::Dummy => (held, just_released, toggle),
//                 StatefulChargedBindingVariant::Held(input) => {
//                     input.update(sources);
//                     (
//                         held || input.is_active(),
//                         just_released || input.just_released(),
//                         toggle,
//                     )
//                 }
//                 StatefulChargedBindingVariant::Toggle(input) => {
//                     input.update(sources);
//                     (held, just_released, toggle || input.just_pressed())
//                 }
//                 StatefulChargedBindingVariant::Preset(timer, input) => {
//                     input.update(sources);
//                     timer.tick(sources.time.delta());
//                     if timer.paused() && input.just_pressed() {
//                         timer.reset();
//                         timer.unpause();
//                     }
//                     if !timer.paused() && timer.finished() {
//                         timer.pause();
//                         (held, true, toggle)
//                     } else if !timer.paused() {
//                         (true, just_released, toggle)
//                     } else {
//                         (held, just_released, toggle)
//                     }
//                 }
//             },
//         );
//         if toggle && self.toggled_on {
//             just_released = true;
//             self.toggled_on = false;
//         } else if toggle {
//             self.toggled_on = true;
//         }
//         if self.toggled_on {
//             held = held || !just_released;
//         }
//         if held {
//             // Currently charging...
//             self.time_active.tick(sources.time.delta());
//             self.charging = true;
//         } else if just_released {
//             // Input just released! Fire the charged event!
//             self.just_released = true;
//             self.charging = false;
//             // Pulsing should break the toggle, if it was toggled.
//             self.toggled_on = false;
//         } else {
//             // It is possible for an active state to expire without firing a release event,
//             // when a conflicting key-chord is pressed. Clean up the timer.
//             self.time_active.reset();
//             self.charging = false;
//             self.just_released = false;
//             self.toggled_on = false;
//         }
//     }
// }
