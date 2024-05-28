# Cross-Reference Analysis Script

This repository contains a Python script and a bash script designed to perform cross-referencing between two Excel datasets:

1. `loveadmin.xlsx` - Contains invoice data.
2. `wholegame.xlsx` - Contains player registration data.

## Features

- Identifies players/teams present in one dataset but not the other.
- Identifies players invoiced for the previous month but not the current month (invoice for July will be created in June).
- Saves the results into an Excel file with three sheets:
  - `Not in Wholegame`: Players/teams in `loveadmin.xlsx` not in `wholegame.xlsx`.
  - `Not in Loveadmin`: Players/teams in `wholegame.xlsx` not in `loveadmin.xlsx`.
  - `Not in Current Month`: Players invoiced for the previous month but not the current month.

## Requirements

- Python 3.x
- `pandas` library
- `openpyxl` library
- `tkinter` library
- `PyInstaller` (for creating a standalone executable)

## Setup

1. Install the required libraries:

```bash
    `pip install pandas openpyxl tk` 
```

1. Save the Python script to a file, for example, `analyze_data.py`.
2. (Optional) To create a standalone executable, use the provided bash script.

## Usage

### Command-Line Mode

Run the Python script with the required arguments:

```bash
`python analyze_data.py --loveadmin_file loveadmin.xlsx --wholegame_file wholegame.xlsx --output_file results.xlsx --team_prefix "Wilpshire Wanderers"` 
```

### GUI Mode

Run the Python script without arguments to use the GUI for file selection:

```bash
python analyze_data.py` 
```

### Create Standalone Executable

Use the provided bash script to create a standalone executable:

1. Save the bash script as `build_executable.sh`.
2. Make the script executable:

```bash
    `chmod +x build_executable.sh` 
```

3. Run the script:

```bash
    `./build_executable.sh -i icon.ico -n MyAnalyzer analyze_data.py` 
```

## Flow Diagrams

### Script Workflow

```mermaid
flowchart TD
    A[Start] --> B{Run Mode}
    B -->|Command-Line| C[Load Data]
    B -->|GUI| D[Select Files via GUI]
    C --> E[Prepare Wholegame Data]
    C --> F[Prepare Loveadmin Data]
    D --> E[Prepare Wholegame Data]
    D --> F[Prepare Loveadmin Data]
    E --> G[Find Not in Wholegame]
    F --> H[Find Not in Loveadmin]
    G --> I[Find Not in Current Month]
    H --> I[Find Not in Current Month]
    I --> J[Save Results]
    J --> K[End]
```

### Sequence Diagram

```mermaid
sequenceDiagram
    actor User
    participant analyze_data.py
    participant CommandLine as Command Line
    participant GUI as GUI
    participant pandas as pandas Library
    participant tkinter as tkinter Library
    participant pd.ExcelWriter as Excel Writer
    participant Log as Logging

    User ->> CommandLine: Run script with arguments
    CommandLine ->> analyze_data.py: Pass file paths and team prefix
    analyze_data.py ->> Log: Log start of command-line mode

    User ->> GUI: Run script without arguments
    GUI ->> analyze_data.py: Initiate GUI mode
    analyze_data.py ->> Log: Log start of GUI mode
    analyze_data.py ->> tkinter: Open file dialog for loveadmin.xlsx
    tkinter ->> analyze_data.py: Return loveadmin.xlsx path
    analyze_data.py ->> tkinter: Open file dialog for wholegame.xlsx
    tkinter ->> analyze_data.py: Return wholegame.xlsx path
    analyze_data.py ->> tkinter: Save output as dialog
    tkinter ->> analyze_data.py: Return output file path
    analyze_data.py ->> tkinter: Prompt for team prefix
    tkinter ->> analyze_data.py: Return team prefix

    analyze_data.py ->> pandas: Load loveadmin.xlsx
    pandas ->> analyze_data.py: Return DataFrame loveadmin_df
    analyze_data.py ->> pandas: Load wholegame.xlsx with header=6
    pandas ->> analyze_data.py: Return DataFrame wholegame_df
    analyze_data.py ->> Log: Log successful data loading

    analyze_data.py ->> analyze_data.py: prepare_wholegame_data(wholegame_df, team_prefix)
    analyze_data.py ->> Log: Log successful preparation of whole game data
    analyze_data.py ->> analyze_data.py: prepare_loveadmin_data(loveadmin_df)
    analyze_data.py ->> Log: Log successful preparation of loveadmin data

    analyze_data.py ->> analyze_data.py: find_not_in_wholegame(loveadmin_df, wholegame_df)
    analyze_data.py ->> Log: Log finding not in whole game data
    analyze_data.py ->> analyze_data.py: find_not_in_loveadmin(wholegame_df, loveadmin_df)
    analyze_data.py ->> Log: Log finding not in loveadmin data
    analyze_data.py ->> analyze_data.py: find_not_in_current_month(loveadmin_df)
    analyze_data.py ->> Log: Log finding not in current month

    analyze_data.py ->> pd.ExcelWriter: Save results to Excel file
    pd.ExcelWriter ->> analyze_data.py: Return confirmation
    analyze_data.py ->> Log: Log successful saving of results

    analyze_data.py ->> User: Notify completion

```

### Function Call Flow

```mermaid

flowchart TD
    A[main] --> B[load_data]
    A --> C[prepare_wholegame_data]
    A --> D[prepare_loveadmin_data]
    A --> E[find_not_in_wholegame]
    A --> F[find_not_in_loveadmin]
    A --> G[find_not_in_current_month]
    A --> H[save_results]
    B --> I[read_excel]
    C --> J[str.replace]
    D --> K[str.extract]
    D --> L[to_datetime]
    E --> M[isin]
    F --> N[isin]
    G --> O[Date Manipulation]
    H --> P[ExcelWriter]
```

## Notes

- Ensure that the `wholegame.xlsx` files have the first 6 rows as metadata. The script uses the `header=6` parameter in `pd.read_excel` to skip these rows.
- The `team_prefix` argument allows you to specify and remove the prefix from team names in the `wholegame.xlsx` file.

---

This README provides a comprehensive overview of the scripts, including setup instructions, usage examples, and visual flow diagrams to illustrate the workflow and function calls.
