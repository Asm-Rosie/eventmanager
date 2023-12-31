typedef struct ExposedStrings {
    const char* summaries;
    const char* descriptions;
    const char* start_dates;
    const char* end_dates;
    const char* uuid;
} ExposedStrings;

void create_new_entry(const char* input_string, const char* input_string2, const char* input_string3);
ExposedStrings expose_strings(void);
void delete_block_call(const char* input);
void does_file_exist();
void edit_content(
                  const char* input1,
                  const char* input2,
                  const char* input3,
                  const char* input4,
                  const char* input5,
                  const char* input6,
                  const char* input7
                  );
void play_ui_sound(const char* sound_file, int disabled);

