ckb_std::entry_simulator!({{project-name | append: "@@SIMULATOR_PLACEHOLDER@@" | remove: "-sim@@SIMULATOR_PLACEHOLDER@@" | replace: "-", "_"}}::program_entry);
