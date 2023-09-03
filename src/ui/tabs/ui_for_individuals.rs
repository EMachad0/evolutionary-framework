use bevy::prelude::*;
use itertools::Itertools;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::selected_individuals::SelectedIndividuals;
use crate::ui::ui_config::SelectionMode;

pub fn ui_for_individuals(world: &mut World, ui: &mut egui::Ui) {
    let mut individuals = world
        .query::<(Entity, &Name, &Fitness)>()
        .iter(world)
        .map(|(e, n, f)| (e, n.clone(), *f))
        .collect_vec();
    individuals.sort_by(|(_, _, a), (_, _, b)| a.get().partial_cmp(&b.get()).unwrap().reverse());
    let ui_config = world
        .resource::<crate::ui::ui_config::UiConfig>()
        .selection_mode;
    let mut selected_entities = world.resource_mut::<SelectedIndividuals>();
    individuals.into_iter().for_each(|(entity, name, fitness)| {
        let text = format!("{} ({:.2})", name, fitness.get());
        if ui
            .selectable_label(selected_entities.contains(&entity), text)
            .clicked()
        {
            match ui_config {
                SelectionMode::Single => selected_entities.replace(entity),
                SelectionMode::Many => selected_entities.select(entity),
            };
        }
    });
}
