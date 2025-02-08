use std::collections::HashMap;

use super::macros::keymap;
use super::{KeyTrie, Mode};
use helix_core::hashmap;

pub fn default() -> HashMap<Mode, KeyTrie> {
    let normal = keymap!({ "Normal mode"
        "m" | "left" => move_char_left,
        "n" | "down" => move_visual_line_down,
        "e" | "up" => move_visual_line_up,
        "i" | "right" => move_char_right,

        "s" => { "Search"
            "s" => find_next_char,
            "t" => find_till_char,
            "w" => search_selection_detect_word_boundaries,
            "W" => search_selection,
        },
        "S" => find_prev_char,
        "T" => till_prev_char,
        "t" => { "Tap"
            "a" => select_all,
            "c" => collapse_selection,

            "f" => ensure_selections_forward,
            "r" => select_regex,
            "s" => split_selection,
            "t" => flip_selections,

            "k" => keep_selections,
            "K" => remove_selections,

            "M" => merge_consecutive_selections,
            "m" => merge_selections,

            "l" => split_selection_on_newline,

            "i" => shrink_selection,
            "o" => expand_selection,

            "n" => select_next_sibling,
            "p" => select_prev_sibling,
        },
        "v" => replace, // new mnemonic: revise
        "V" => replace_with_yanked,
        "'" =>  repeat_last_motion,
        "\"" =>  repeat_last_motion_reverse,

        "~" => switch_case,
        "`" => switch_to_lowercase,
        "A-`" => switch_to_uppercase,

        "home" => goto_line_start,
        "end" => goto_line_end,
        "M" => goto_line_start,
        "I" => goto_line_end,
        "^" => goto_first_nonwhitespace,
        "$" => goto_line_end,

        "w" => move_next_word_start,
        "b" => move_prev_word_start,
        "f" => move_next_word_end, // new mnemonic: f/F = far word/WORD

        "W" => move_next_long_word_start,
        "B" => move_prev_long_word_start,
        "F" => move_next_long_word_end,

        "r" => select_mode, // new mnemonic: r/R = range
        "G" => goto_last_line, // old habits die hard
        "j" => { "Jump"
            "j" => goto_line,
            "s" => move_parent_node_start,
            "e" => move_parent_node_end,
            "(" => rotate_selections_first,
            ")" => rotate_selections_last,
        },
        "g" => { "Goto"
            "g" => goto_file_start,
            "|" => goto_column,
            "e" => goto_last_line,
            "f" => goto_file,
            "m" => goto_line_start,
            "i" => goto_line_end,
            "d" => goto_definition,
            "D" => goto_declaration,
            "y" => goto_type_definition,
            "r" => goto_reference,
            "I" => goto_implementation,
            "t" => goto_window_top,
            "c" => goto_window_center,
            "b" => goto_window_bottom,
            "A" => goto_last_accessed_file,
            "M" => goto_last_modified_file,
            "n" => goto_next_buffer,
            "p" => goto_previous_buffer,
            "." => goto_last_modification,
            "w" => goto_word,
        },
        ":" => command_mode,

        "l" => insert_mode,
        "L" => insert_at_line_start,
        "a" => append_mode,
        "A" => insert_at_line_end,
        "o" => open_below,
        "O" => open_above,

        "d" => delete_selection,
        "A-d" => delete_selection_noyank,
        "c" => change_selection,
        "A-c" => change_selection_noyank,

        "C" => copy_selection_on_next_line,
        "A-C" => copy_selection_on_prev_line,

        "x" => extend_line_below,
        "X" => extend_line_above,

        "%" => match_brackets, // easier reach on my keyboard due to layers
        "k" => { "Knit" // new mnemonic: KNIT mode
            "k" => match_brackets,
            "s" => surround_add,
            "r" => surround_replace,
            "d" => surround_delete,
            "a" => select_textobject_around,
            "i" => select_textobject_inner,
        },
        "[" => { "Left bracket"
            "d" => goto_prev_diag,
            "D" => goto_first_diag,
            "g" => goto_prev_change,
            "G" => goto_first_change,
            "f" => goto_prev_function,
            "t" => goto_prev_class,
            "a" => goto_prev_parameter,
            "c" => goto_prev_comment,
            "e" => goto_prev_entry,
            "T" => goto_prev_test,
            "p" => goto_prev_paragraph,
            "x" => goto_prev_xml_element,
            "space" => add_newline_above,
        },
        "]" => { "Right bracket"
            "d" => goto_next_diag,
            "D" => goto_last_diag,
            "g" => goto_next_change,
            "G" => goto_last_change,
            "f" => goto_next_function,
            "t" => goto_next_class,
            "a" => goto_next_parameter,
            "c" => goto_next_comment,
            "e" => goto_next_entry,
            "T" => goto_next_test,
            "p" => goto_next_paragraph,
            "x" => goto_next_xml_element,
            "space" => add_newline_below,
        },

        "/" => search,
        "?" => rsearch,
        "h" => search_next,
        "H" => search_prev,
        "*" => search_selection_detect_word_boundaries,

        "u" => undo,
        "U" => redo,

        "y" => yank,
        "Y" => yank_joined,
        // yank_all
        "p" => paste_after,
        // paste_all
        "P" => paste_before,

        "Q" => record_macro,
        "q" => replay_macro,

        ">" => indent,
        "<" => unindent,
        "J" => join_selections,
        "A-J" => join_selections_space,

        "," => keep_primary_selection,
        ";" => remove_primary_selection,

        "=" => align_selections,
        "_" => trim_selections,

        "(" => rotate_selections_backward,
        ")" => rotate_selections_forward,
        "A-(" => rotate_selection_contents_backward,
        "A-)" => rotate_selection_contents_forward,

        "esc" => normal_mode,
        "C-E" | "C-b" | "pageup" => page_up,
        "C-N" | "C-f" | "pagedown" => page_down,
        "C-u" => page_cursor_half_up,
        "C-d" => page_cursor_half_down,

        "C-w" => { "Window"
            "C-w" | "w" => rotate_view,
            "C-s" | "s" => hsplit,
            "C-v" | "v" => vsplit,
            "C-t" | "t" => transpose_view,
            "f" => goto_file_vsplit,
            "F" => goto_file_hsplit,
            "C-k" | "k" => wclose,
            "C-q" | "q" => wclose,
            "C-o" | "o" => wonly,
            "C-m" | "m" | "left" => jump_view_left,
            "C-n" | "n" | "down" => jump_view_down,
            "C-e" | "e" | "up" => jump_view_up,
            "C-i" | "i" | "right" => jump_view_right,
            "M" => swap_view_left,
            "E" => swap_view_down,
            "N" => swap_view_up,
            "I" => swap_view_right,
            "b" => { "New split scratch buffer"
                "C-s" | "s" => hsplit_new,
                "C-b" | "b" | "C-v" | "v" => vsplit_new,
            },
        },

        // z family for save/restore/combine from/to sels from register

        "C-i" | "tab" => jump_forward, // tab == <C-i>
        "C-o" => jump_backward,
        "C-s" => save_selection,
        "C-l" => align_view_top, // sorta like Emacs

        "space" => { "Space"
            "f" => { "File"
                "f" => file_picker,
                "F" => file_picker_in_current_directory,
            },
            "e" => file_explorer,
            "E" => file_explorer_in_current_buffer_directory,
            "b" => { "Buffer"
                "b" => buffer_picker,
            },
            "r" => { "Rapid"
                "_" => no_op, // placeholder
            },
            "j" => jumplist_picker,
            "g" => changed_file_picker,
            "'" => last_picker,
            "G" => { "Debug (experimental)" sticky=true
                "l" => dap_launch,
                "r" => dap_restart,
                "b" => dap_toggle_breakpoint,
                "c" => dap_continue,
                "h" => dap_pause,
                "i" => dap_step_in,
                "o" => dap_step_out,
                "n" => dap_next,
                "v" => dap_variables,
                "t" => dap_terminate,
                "C-c" => dap_edit_condition,
                "C-l" => dap_edit_log,
                "s" => { "Switch"
                    "t" => dap_switch_thread,
                    "f" => dap_switch_stack_frame,
                },
                "e" => dap_enable_exceptions,
                "E" => dap_disable_exceptions,
            },
            "w" => { "Window"
                "C-w" | "w" => rotate_view,
                "C-s" | "s" => hsplit,
                "C-v" | "v" => vsplit,
                "C-t" | "t" => transpose_view,
                "f" => goto_file_vsplit,
                "F" => goto_file_hsplit,
                "C-k" | "k" => wclose,
                "C-q" | "q" => wclose,
                "C-o" | "o" => wonly,
                "C-m" | "m" | "left" => jump_view_left,
                "C-n" | "n" | "down" => jump_view_down,
                "C-e" | "e" | "up" => jump_view_up,
                "C-i" | "i" | "right" => jump_view_right,
                "M" => swap_view_left,
                "E" => swap_view_down,
                "N" => swap_view_up,
                "I" => swap_view_right,
                "b" => { "New split scratch buffer"
                    "C-s" | "s" => hsplit_new,
                    "C-b" | "b" | "C-v" | "v" => vsplit_new,
                },
            },
            "/" => global_search,
            "?" => command_palette,
            "c" => { "Comments"
                "c" => toggle_comments,
                "C" => toggle_block_comments,
                "l" => toggle_line_comments,
            },
            "p" => { "Project"
                "a" => code_action,
                "d" => diagnostics_picker,
                "D" => workspace_diagnostics_picker,
                "k" => hover,
                "r" => rename_symbol,
                "R" => select_references_to_symbol_under_cursor,
                "s" => lsp_or_syntax_symbol_picker,
                "S" => lsp_or_syntax_workspace_symbol_picker,
            },
            "t" => { "Toggle"
                "_" => no_op, // placeholder
            },
            "x" => { "Text manipulation"
                "=" => format_selections,
                "p" => paste_clipboard_after,
                "P" => paste_clipboard_before,
                "y" => yank_to_clipboard,
                "Y" => yank_joined_to_clipboard,
                "V" => replace_selections_with_clipboard,
            },
            "q" => { "Quit"
                "_" => no_op, // placeholder
            },
            "z" => { "Background/Shell"
                "a" => shell_append_output,
                "i" => shell_insert_output,
                "k" => shell_keep_pipe,
                "p" => shell_pipe,
                "P" => shell_pipe_to,
                "z" => suspend,
            },
        },
        "z" => { "View"
            "z" | "c" => align_view_center,
            "t" => align_view_top,
            "b" => align_view_bottom,
            "m" => align_view_middle,
            "e" | "up" => scroll_up,
            "n" | "down" => scroll_down,
            "C-E" | "C-b" | "pageup" => page_up,
            "C-N" | "C-f" | "pagedown" => page_down,
            "C-u" | "backspace" => page_cursor_half_up,
            "C-d" | "space" => page_cursor_half_down,

            "/" => search,
            "?" => rsearch,
            "h" => search_next,
            "H" => search_prev,
        },
        "Z" => { "View" sticky=true
            "z" | "c" => align_view_center,
            "t" => align_view_top,
            "b" => align_view_bottom,
            "m" => align_view_middle,
            "e" | "up" => scroll_up,
            "n" | "down" => scroll_down,
            "C-E" | "C-b" | "pageup" => page_up,
            "C-N" | "C-f" | "pagedown" => page_down,
            "C-u" | "backspace" => page_cursor_half_up,
            "C-d" | "space" => page_cursor_half_down,

            "/" => search,
            "?" => rsearch,
            "h" => search_next,
            "H" => search_prev,
        },

        "&" => select_register,

        "+" => increment,
        "minus" => decrement,
    });
    let mut select = normal.clone();
    select.merge_nodes(keymap!({ "Select mode"
        "m" | "left" => extend_char_left,
        "n" | "down" => extend_visual_line_down,
        "e" | "up" => extend_visual_line_up,
        "i" | "right" => extend_char_right,

        "E" => page_cursor_half_up,
        "N" => page_cursor_half_down,

        "w" => extend_next_word_start,
        "b" => extend_prev_word_start,
        "f" => extend_next_word_end,
        "W" => extend_next_long_word_start,
        "B" => extend_prev_long_word_start,
        "F" => extend_next_long_word_end,

        "j" => { "Jump"
            "b" => extend_parent_node_start,
            "e" => extend_parent_node_end,
        },

        "h" => extend_search_next,
        "H" => extend_search_prev,

        "s" => { "Search"
            "s" => extend_next_char,
            "S" => extend_prev_char,
            "t" => extend_till_char,
            "T" => extend_till_prev_char,
        },
        "'" =>  extend_repeat_last_motion,
        "\"" =>  extend_repeat_last_motion_reverse,

        "home" => extend_to_line_start,
        "end" => extend_to_line_end,
        "esc" => exit_select_mode,

        "r" => normal_mode,
        "G" => extend_to_last_line,
        "g" => { "Goto"
            "g" => extend_to_file_start,
            "|" => extend_to_column,
            "e" => extend_to_last_line,
            "w" => extend_to_word,
        },
    }));
    let insert = keymap!({ "Insert mode"
        "esc" => normal_mode,

        "C-s" => commit_undo_checkpoint,
        "C-x" => completion,
        "C-r" => insert_register,

        "C-w" | "A-backspace" => delete_word_backward,
        "A-d" | "A-del" => delete_word_forward,
        "C-u" => kill_to_line_start,
        "C-k" => kill_to_line_end,
        "C-h" | "backspace" | "S-backspace" => delete_char_backward,
        "C-d" | "del" => delete_char_forward,
        "C-j" | "ret" => insert_newline,
        "tab" => smart_tab,
        "S-tab" => insert_tab,

        "up" => move_visual_line_up,
        "down" => move_visual_line_down,
        "left" => move_char_left,
        "right" => move_char_right,
        "pageup" => page_up,
        "pagedown" => page_down,
        "home" => goto_line_start,
        "end" => goto_line_end_newline,
        // Emacs like
        "C-a" => goto_line_start,
        "C-e" => goto_line_end_newline,
    });
    hashmap!(
        Mode::Normal => normal,
        Mode::Select => select,
        Mode::Insert => insert,
    )
}
