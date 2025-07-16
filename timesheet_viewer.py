import streamlit as st
import pandas as pd
from datetime import datetime, timedelta, date
import json
from pathlib import Path
import plotly.express as px
import plotly.graph_objects as go
import calendar
import numpy as np

# Page config
st.set_page_config(page_title="Timesheet Analytics", layout="wide", initial_sidebar_state="expanded")

# Custom CSS for dark theme
st.markdown("""
    <style>
    /* Main container and background */
    .main {
        padding: 0rem 1rem;
        background-color: #0e1117;
        color: #fafafa;
    }
    
    /* Metrics styling */
    .stMetric {
        background-color: #1e2127;
        padding: 10px;
        border-radius: 5px;
        border: 1px solid #2d3035;
    }
    
    /* Headers and text */
    h1, h2, h3, h4, h5, h6 {
        color: #fafafa !important;
    }
    
    /* Table styling */
    .timesheet {
        font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", Roboto;
        background-color: #1e2127;
        color: #fafafa;
    }
    
    .total-row {
        font-weight: bold;
        background-color: #2d3035;
        color: #fafafa;
    }
    
    .total-col {
        font-weight: bold;
        border-left: 2px solid #2d3035;
        color: #fafafa;
    }
    
    .date-header {
        font-size: 0.9em;
        color: #c0c0c0;
    }
    
    /* Expander styling */
    .streamlit-expanderHeader {
        background-color: #1e2127 !important;
        color: #fafafa !important;
    }
    
    /* DataFrames and tables */
    .dataframe {
        background-color: #1e2127 !important;
        color: #fafafa !important;
    }
    
    /* Sidebar */
    .css-1d391kg {
        background-color: #1e2127;
    }
    
    /* Input fields */
    .stTextInput>div>div>input {
        background-color: #2d3035;
        color: #fafafa;
    }
    
    /* Buttons */
    .stButton>button {
        background-color: #e31837;
        color: #ffffff;
        border: none;
    }
    
    .stButton>button:hover {
        background-color: #f32847;
        color: #ffffff;
        border: none;
    }
    
    /* Download button */
    .stDownloadButton>button {
        background-color: #2d3035;
        color: #fafafa;
        border: 1px solid #4d5055;
    }
    
    /* Success/Info/Warning messages */
    .stSuccess, .stInfo, .stWarning, .stError {
        background-color: #1e2127;
        color: #fafafa;
        border: 1px solid #2d3035;
    }
    
    /* Date picker */
    .stDateInput>div>div>input {
        background-color: #2d3035;
        color: #fafafa;
        border: 1px solid #4d5055;
    }
    
    /* General containers */
    .css-1r6slb0, .css-12w0qpk {
        background-color: #1e2127;
        border: 1px solid #2d3035;
    }
    
    /* Remove white background from empty cells */
    .stDataFrame td:empty {
        background-color: #1e2127 !important;
    }
    
    </style>
""", unsafe_allow_html=True)

def load_timesheet_data(data_dir):
    """Load and process timesheet data from CSV files only"""
    all_data = []
    
    # Only look for CSV files
    csv_files = list(data_dir.glob("project_times_*.csv"))
    st.sidebar.write(f"Found {len(csv_files)} CSV files")
    
    for file in csv_files:
        try:
            st.sidebar.write(f"\nReading {file.name}")
            df_csv = pd.read_csv(file)
            
            # Convert CSV data to our format
            for _, row in df_csv.iterrows():
                try:
                    # Split DateTime into date only
                    date_str = row['DateTime'].split()[0]
                    
                    # Handle both project codes and internal codes
                    project = row['Project'].strip()
                    
                    # Only include entries with more than 0.2 hours
                    hours = float(row['Total Time (Hours)'])
                    if hours >= 0.2:
                        entry = {
                            'Date': date_str,
                            'Project': project,
                            'Duration': hours
                        }
                        all_data.append(entry)
                except Exception as e:
                    st.sidebar.error(f"Error processing row: {row}\nError: {str(e)}")
                    continue
                    
            st.sidebar.write(f"Added {len(df_csv)} entries from {file.name}")
        except Exception as e:
            st.sidebar.error(f"Error reading {file.name}: {str(e)}")
            continue
    
    if not all_data:
        st.sidebar.error("No CSV data found")
        return pd.DataFrame()
    
    # Create DataFrame
    df = pd.DataFrame(all_data)
    
    # Convert date to datetime
    df['Date'] = pd.to_datetime(df['Date'])
    
    # Group by Date and Project to combine any duplicate entries
    df = df.groupby(['Date', 'Project'])['Duration'].sum().reset_index()
    
    # Filter again after grouping to ensure combined entries still meet threshold
    df = df[df['Duration'] >= 0.2]
    
    # Sort by date
    df = df.sort_values('Date', ascending=False)
    
    # Show summary
    st.sidebar.write("\nData Summary:")
    st.sidebar.write(f"Date range: {df['Date'].min().date()} to {df['Date'].max().date()}")
    st.sidebar.write("Total projects:", df['Project'].nunique())
    st.sidebar.write("Total entries:", len(df))
    
    return df

def calculate_metrics(df):
    """Calculate key performance metrics"""
    if df.empty:
        return {}
    
    total_hours = df['Duration'].sum()
    unique_projects = df['Project'].nunique()
    avg_daily_hours = total_hours / df['Date'].nunique()
    
    # Calculate project distribution
    project_hours = df.groupby('Project')['Duration'].sum().sort_values(ascending=False)
    
    # Calculate productivity score (based on consistency and task completion)
    daily_hours = df.groupby('Date')['Duration'].sum()
    consistency_score = 1 - daily_hours.std() / daily_hours.mean() if len(daily_hours) > 1 else 0
    
    return {
        'total_hours': total_hours,
        'unique_projects': unique_projects,
        'avg_daily_hours': avg_daily_hours,
        'project_distribution': project_hours,
        'consistency_score': min(consistency_score, 1) * 100
    }

def create_burndown_chart(df, start_date, end_date):
    """Create a burndown chart showing cumulative hours"""
    if df.empty:
        return None
    
    date_range = pd.date_range(start_date, end_date)
    daily_hours = df.groupby('Date')['Duration'].sum()
    cumulative_hours = daily_hours.cumsum()
    
    fig = go.Figure()
    fig.add_trace(go.Scatter(
        x=cumulative_hours.index,
        y=cumulative_hours.values,
        mode='lines+markers',
        name='Actual Hours',
        line=dict(color='#0066cc')
    ))
    
    # Add ideal line
    if len(cumulative_hours) > 0:
        ideal_line = np.linspace(0, cumulative_hours.iloc[-1], len(date_range))
        fig.add_trace(go.Scatter(
            x=date_range,
            y=ideal_line,
            mode='lines',
            name='Ideal Progress',
            line=dict(dash='dash', color='#999999')
        ))
    
    fig.update_layout(
        title='Cumulative Hours Progress',
        xaxis_title='Date',
        yaxis_title='Hours',
        showlegend=True
    )
    return fig

def main():
    st.title("Timesheet Viewer")
    
    # Fix path handling for Windows
    data_dir = Path.cwd() / "timesheetorg" / "timesheet_data"
    if not data_dir.exists():
        data_dir = Path("C:/temp/MalceodJordantimesheet/timesheetorg/timesheet_data")
        if not data_dir.exists():
            st.error(f"Data directory not found at: {data_dir}")
            st.info("Please make sure you're running the script from the correct directory")
            return
    
    df = load_timesheet_data(data_dir)
    if df.empty:
        st.warning("No timesheet entries found.")
        return

    # Get min and max dates from data
    min_date = df['Date'].min().date()
    max_date = df['Date'].max().date()
    
    # Find the Monday of the week containing min_date
    min_monday = min_date - timedelta(days=min_date.weekday())
    # Find the Friday of the week containing max_date
    max_friday = max_date + timedelta(days=4-max_date.weekday())
    
    # Debug date information
    st.sidebar.write("Debug Date Info:")
    st.sidebar.write(f"Min date in data: {min_date}")
    st.sidebar.write(f"Max date in data: {max_date}")
    
    # Find the most recent Monday
    default_date = max_date - timedelta(days=max_date.weekday())
    
    # Debug default date
    st.sidebar.write(f"Using default date (Monday): {default_date}")
    
    try:
        # Week selection
        selected_date = st.date_input(
            "Select Week",
            value=default_date,
            min_value=min_monday,
            max_value=max_friday,
            key="date_picker"
        )
        
        # Always start from Monday of the selected week
        week_start = selected_date - timedelta(days=selected_date.weekday())
        # End on Friday
        week_end = week_start + timedelta(days=4)
        
        # Debug week range
        st.sidebar.write(f"Week range: {week_start} to {week_end}")
        
        # Filter data for selected week
        mask = (df['Date'].dt.date >= week_start) & (df['Date'].dt.date <= week_end)
        week_data = df[mask].copy()
        
        if week_data.empty:
            st.warning("No data available for the selected week.")
            return

        # Create daily columns for the week (Monday to Friday)
        days = [(week_start + timedelta(days=i)) for i in range(5)]  # 5 days instead of 7
        day_names = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri']
        day_dates = [d.strftime('%b %d') for d in days]
        
        # Convert dates to datetime for proper pivoting
        week_data['Date'] = pd.to_datetime(week_data['Date']).dt.date
        
        # Group data by project and day
        pivot_data = pd.pivot_table(
            week_data,
            index='Project',
            columns='Date',
            values='Duration',
            aggfunc='sum',
            fill_value=0
        )

        # Calculate row totals and sort by total hours
        pivot_data['Task Total'] = pivot_data.sum(axis=1)
        pivot_data = pivot_data.sort_values('Task Total', ascending=False)
        
        # Calculate column totals
        daily_totals = pivot_data.sum()

        # Style the table
        st.markdown("""
            <style>
            .timesheet {
                font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", Roboto;
            }
            .total-row {
                font-weight: bold;
                background-color: #f0f2f6;
            }
            .total-col {
                font-weight: bold;
                border-left: 2px solid #ddd;
            }
            .date-header {
                font-size: 0.9em;
                color: #666;
            }
            </style>
        """, unsafe_allow_html=True)

        # Display week range
        st.subheader(f"Week of {week_start.strftime('%B %d')} to {week_end.strftime('%B %d, %Y')}")

        # Create the main timesheet table
        col1, col2 = st.columns([8, 1])
        with col1:
            # Headers
            header_cols = st.columns([3] + [1]*len(days) + [1])
            with header_cols[0]:
                st.write("Project")
            for i, (name, date_str) in enumerate(zip(day_names, day_dates), 1):
                with header_cols[i]:
                    st.markdown(f"<div class='date-header'>{name}<br>{date_str}</div>", unsafe_allow_html=True)
            with header_cols[-1]:
                st.write("Total")

            # Data rows
            for project in pivot_data.index:
                cols = st.columns([3] + [1]*len(days) + [1])
                with cols[0]:
                    st.write(project)
                for i, day in enumerate(days, 1):
                    with cols[i]:
                        value = pivot_data.get(day, {}).get(project, 0)
                        st.write(f"{value:.1f}" if value > 0 else "")
                with cols[-1]:
                    st.write(f"{pivot_data.loc[project, 'Task Total']:.1f}")

            # Totals row with dark theme styling
            total_cols = st.columns([3] + [1]*len(days) + [1])
            with total_cols[0]:
                st.markdown("<div style='font-weight: bold; background-color: #2d3035; color: #fafafa; padding: 8px; border-radius: 4px;'>Daily Total:</div>", unsafe_allow_html=True)
            for i, day in enumerate(days, 1):
                with total_cols[i]:
                    value = daily_totals.get(day, 0)
                    if value > 0:
                        st.markdown(f"<div style='font-weight: bold; background-color: #2d3035; color: #fafafa; padding: 8px; border-radius: 4px;'>{value:.1f}</div>", unsafe_allow_html=True)
                    else:
                        st.write("")
            with total_cols[-1]:
                st.markdown(f"<div style='font-weight: bold; background-color: #2d3035; color: #fafafa; padding: 8px; border-radius: 4px;'>{daily_totals['Task Total']:.1f}</div>", unsafe_allow_html=True)

        # Export options
        with st.sidebar:
            st.download_button(
                "Download Timesheet",
                data=pivot_data.to_csv(),
                file_name=f"timesheet_{week_start.strftime('%Y-%m-%d')}.csv",
                mime="text/csv"
            )
            
    except Exception as e:
        st.error(f"Error: {str(e)}")
        st.sidebar.error(f"Debug: {str(e)}")

if __name__ == "__main__":
    main() 