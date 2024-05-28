import pandas as pd
from datetime import datetime
import argparse
import tkinter as tk
from tkinter import filedialog, simpledialog
import logging

# Setup logging to both file and standard output
logging.basicConfig(level=logging.INFO, 
                    format='%(asctime)s - %(levelname)s - %(message)s',
                    handlers=[
                        logging.FileHandler("analysis.log"),
                        logging.StreamHandler()
                    ])

"""
This script performs cross-referencing between two Excel datasets:
1. 'loveadmin.xlsx' - containing invoice data.
2. 'wholegame.xlsx' - containing player registration data.

The script can be run in two modes:
1. Command-line mode: Accepts file paths as command-line arguments.
2. GUI mode: Opens a file dialog to select files if no arguments are passed.

The script performs the following analyses:
1. Identifies players/teams present in one dataset but not the other.
2. Identifies players invoiced for the previous month but not the current month (invoice for July will be created in June).

The results are saved into an Excel file with three sheets:
1. Not in Wholegame: Players/teams in 'loveadmin.xlsx' not in 'wholegame.xlsx'.
2. Not in Loveadmin: Players/teams in 'wholegame.xlsx' not in 'loveadmin.xlsx'.
3. Not in Current Month: Players invoiced for the previous month but not the current month.

Note:
- The 'header=6' parameter in 'pd.read_excel' is used to skip the first 6 rows of metadata in 'wholegame.xlsx'. Ensure all 'wholegame.xlsx' files have this structure.
"""

def load_data(loveadmin_file, wholegame_file):
    """
    Load data from the given Excel files.
    
    Args:
        loveadmin_file (str): Path to the loveadmin Excel file.
        wholegame_file (str): Path to the wholegame Excel file.
    
    Returns:
        tuple: DataFrames loaded from the Excel files.
    """
    try:
        loveadmin_df = pd.read_excel(loveadmin_file)
        
        # Ensure the header=6 parameter is applicable
        # This parameter skips the first 6 rows of metadata in 'wholegame.xlsx'
        wholegame_df = pd.read_excel(wholegame_file, header=6)
        
        logging.info("Data loaded successfully from Excel files.")
        return loveadmin_df, wholegame_df
    except Exception as e:
        logging.error(f"Error loading data: {e}")
        raise

def prepare_wholegame_data(df, team_prefix):
    """
    Prepare whole game data by creating a 'Name' column and normalizing 'Team Name'.
    
    Args:
        df (DataFrame): DataFrame containing whole game data.
        team_prefix (str): The prefix to be removed from the team names.
    
    Returns:
        DataFrame: Prepared DataFrame with 'Name' and normalized 'Team Name'.
    """
    try:
        df['Name'] = df['First names'] + ' ' + df['Surname']
        df['Team Name'] = df['Team'].str.replace(f'{team_prefix} ', '', regex=False)
        logging.info("Whole game data prepared successfully.")
        return df
    except Exception as e:
        logging.error(f"Error preparing whole game data: {e}")
        raise

def prepare_loveadmin_data(df):
    """
    Prepare loveadmin data by extracting 'Team Name' and converting 'Date' to datetime.
    
    Args:
        df (DataFrame): DataFrame containing loveadmin data.
    
    Returns:
        DataFrame: Prepared DataFrame with extracted 'Team Name' and converted 'Date'.
    """
    try:
        df['Team Name'] = df['Product'].str.extract(r'(.+?) \(\*\)', expand=False)
        df['Date'] = pd.to_datetime(df['Date'])
        logging.info("Loveadmin data prepared successfully.")
        return df
    except Exception as e:
        logging.error(f"Error preparing loveadmin data: {e}")
        raise

def find_not_in_wholegame(loveadmin_df, wholegame_df):
    """
    Find records in loveadmin that are not in whole game data.
    
    Args:
        loveadmin_df (DataFrame): DataFrame containing loveadmin data.
        wholegame_df (DataFrame): DataFrame containing whole game data.
    
    Returns:
        DataFrame: Records in loveadmin not present in whole game data.
    """
    try:
        result = loveadmin_df[~loveadmin_df['Name'].isin(wholegame_df['Name']) | ~loveadmin_df['Team Name'].isin(wholegame_df['Team Name'])]
        logging.info("Records in loveadmin not present in whole game data found successfully.")
        return result
    except Exception as e:
        logging.error(f"Error finding records not in whole game: {e}")
        raise

def find_not_in_loveadmin(wholegame_df, loveadmin_df):
    """
    Find records in whole game data that are not in loveadmin.
    
    Args:
        wholegame_df (DataFrame): DataFrame containing whole game data.
        loveadmin_df (DataFrame): DataFrame containing loveadmin data.
    
    Returns:
        DataFrame: Records in whole game data not present in loveadmin.
    """
    try:
        result = wholegame_df[~wholegame_df['Name'].isin(loveadmin_df['Name']) | ~wholegame_df['Team Name'].isin(loveadmin_df['Team Name'])]
        logging.info("Records in whole game data not present in loveadmin found successfully.")
        return result
    except Exception as e:
        logging.error(f"Error finding records not in loveadmin: {e}")
        raise

def find_not_in_current_month(loveadmin_df):
    """
    Find players invoiced for the previous month but not the current month (invoice for July created in June).
    
    Args:
        loveadmin_df (DataFrame): DataFrame containing loveadmin data.
    
    Returns:
        DataFrame: Players invoiced for the previous month but not the current month.
    """
    try:
        # Get the current and previous month dates
        current_month = loveadmin_df['Date'].max().replace(day=1)
        previous_month = (current_month - pd.DateOffset(months=1))
        two_months_ago = (previous_month - pd.DateOffset(months=1))
        
        current_month_data = loveadmin_df[(loveadmin_df['Date'] >= previous_month) & (loveadmin_df['Date'] < current_month)]
        previous_month_data = loveadmin_df[(loveadmin_df['Date'] >= two_months_ago) & (loveadmin_df['Date'] < previous_month)]
        
        prev_month_players = set(previous_month_data['Name'])
        current_month_players = set(current_month_data['Name'])
        not_in_current_month = prev_month_players - current_month_players
        
        result = previous_month_data[previous_month_data['Name'].isin(not_in_current_month)]
        logging.info("Players invoiced for the previous month but not the current month found successfully.")
        return result
    except Exception as e:
        logging.error(f"Error finding players not invoiced in current month: {e}")
        raise

def save_results(not_in_wholegame, not_in_loveadmin, not_in_current_month, output_file):
    """
    Save the results to different sheets in the same Excel file.
    
    Args:
        not_in_wholegame (DataFrame): Records in loveadmin not present in whole game data.
        not_in_loveadmin (DataFrame): Records in whole game data not present in loveadmin.
        not_in_current_month (DataFrame): Players invoiced for the previous month but not the current month.
        output_file (str): Path to the output Excel file.
    """
    try:
        with pd.ExcelWriter(output_file) as writer:
            not_in_wholegame.to_excel(writer, sheet_name='Not in Wholegame', index=False)
            not_in_loveadmin.to_excel(writer, sheet_name='Not in Loveadmin', index=False)
            not_in_current_month.to_excel(writer, sheet_name='Not in Current Month', index=False)
        logging.info(f"Results saved successfully to '{output_file}'.")
    except Exception as e:
        logging.error(f"Error saving results: {e}")
        raise

def main(loveadmin_file, wholegame_file, output_file, team_prefix):
    """
    Main function to load, process, and save data.
    
    Args:
        loveadmin_file (str): Path to the loveadmin Excel file.
        wholegame_file (str): Path to the wholegame Excel file.
        output_file (str): Path to the output Excel file.
        team_prefix (str): The prefix to be removed from the team names in wholegame.
    """
    try:
        loveadmin_df, wholegame_df = load_data(loveadmin_file, wholegame_file)
        wholegame_df = prepare_wholegame_data(wholegame_df, team_prefix)
        loveadmin_df = prepare_loveadmin_data(loveadmin_df)
        
        not_in_wholegame = find_not_in_wholegame(loveadmin_df, wholegame_df)
        not_in_loveadmin = find_not_in_loveadmin(wholegame_df, loveadmin_df)
        not_in_current_month = find_not_in_current_month(loveadmin_df)
        
        save_results(not_in_wholegame, not_in_loveadmin, not_in_current_month, output_file)
    except Exception as e:
        logging.error(f"Error in main function: {e}")
        raise

def gui_mode():
    """
    GUI mode to select files and output location.
    """
    try:
        root = tk.Tk()
        root.withdraw()  # Hide the root window
        
        loveadmin_file = filedialog.askopenfilename(title='Select loveadmin.xlsx', filetypes=[('Excel files', '*.xlsx')])
        if not loveadmin_file:
            logging.warning("No loveadmin file selected. Exiting.")
            return
        
        wholegame_file = filedialog.askopenfilename(title='Select wholegame.xlsx', filetypes=[('Excel files', '*.xlsx')])
        if not wholegame_file:
            logging.warning("No wholegame file selected. Exiting.")
            return
        
        output_file = filedialog.asksaveasfilename(title='Save output as', defaultextension='.xlsx', filetypes=[('Excel files', '*.xlsx')])
        if not output_file:
            logging.warning("No output file selected. Exiting.")
            return
        
        team_prefix = simpledialog.askstring("Input", "Enter the team prefix to be removed:")
        if not team_prefix:
            logging.warning("No team prefix entered. Exiting.")
            return
        
        main(loveadmin_file, wholegame_file, output_file, team_prefix)
    except Exception as e:
        logging.error(f"Error in GUI mode: {e}")
        raise

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Cross-reference loveadmin and wholegame data.')
    parser.add_argument('--loveadmin_file', help='Path to the loveadmin.xlsx file')
    parser.add_argument('--wholegame_file', help='Path to the wholegame.xlsx file')
    parser.add_argument('--output_file', help='Path to the output Excel file')
    parser.add_argument('--team_prefix', help='Prefix to be removed from team names in wholegame')

    args = parser.parse_args()
    
    if args.loveadmin_file and args.wholegame_file and args.output_file and args.team_prefix:
        logging.info("Running in command-line mode.")
        main(args.loveadmin_file, args.wholegame_file, args.output_file, args.team_prefix)
    else:
        logging.info("Running in GUI mode.")
        gui_mode()
