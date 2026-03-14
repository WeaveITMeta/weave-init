// =============================================================================
// App - Application state machine and screen management for the wizard
// =============================================================================
//
// Table of Contents:
// - AppState: Current screen/phase of the wizard
// - App: Root application struct holding all state
// - Screen navigation (next, previous, handle input)
// - Main render dispatch (delegates to screen renderers)
// - Main event loop (crossterm events → state transitions)
// =============================================================================

use super::widgets::SelectionListState;
use crate::core::manifest::WeaveManifest;
use crate::core::selections::UserSelections;

/// Which screen the wizard is currently showing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    /// Welcome screen with logo
    Welcome,

    /// Category selection screen (index into the category order)
    Selection(usize),

    /// Summary screen showing all choices before scaffolding
    Summary,

    /// Progress screen during scaffolding
    Progress,

    /// Completion screen with next steps
    Complete,
}

/// Root application state for the interactive wizard
pub struct App {
    /// Current screen being displayed
    pub screen: AppScreen,

    /// User's selections throughout the wizard
    pub selections: UserSelections,

    /// Selection list states for each category screen (one per category)
    pub list_states: Vec<SelectionListState>,

    /// Category keys in wizard display order
    pub categories: Vec<String>,
}

impl App {
    /// Create a new App from a parsed manifest and project name
    pub fn new(manifest: WeaveManifest, project_name: String) -> Self {
        let categories: Vec<String> = WeaveManifest::category_order()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Build a SelectionListState for each category from the manifest
        let list_states: Vec<SelectionListState> = categories
            .iter()
            .map(|category| {
                let entries = manifest.get_category_entries(category);
                let keys: Vec<String> = entries.iter().map(|(k, _)| (*k).clone()).collect();
                let labels: Vec<String> = entries.iter().map(|(_, e)| e.label.clone()).collect();
                let descriptions: Vec<String> = entries
                    .iter()
                    .map(|(_, e)| {
                        e.description
                            .clone()
                            .unwrap_or_else(|| "No description available.".to_string())
                    })
                    .collect();
                let mode = UserSelections::selection_mode_for(category);
                SelectionListState::new(keys, labels, descriptions, mode)
            })
            .collect();

        // Filter out categories that have zero entries in the manifest
        let (categories, list_states): (Vec<String>, Vec<SelectionListState>) = categories
            .into_iter()
            .zip(list_states.into_iter())
            .filter(|(_, state)| !state.keys.is_empty())
            .unzip();

        Self {
            screen: AppScreen::Welcome,
            selections: UserSelections::new(project_name),
            list_states,
            categories,
        }
    }
}
