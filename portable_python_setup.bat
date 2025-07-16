@echo off
echo Setting up portable Python environment...

:: Create portable_python directory
mkdir portable_python 2>nul
cd portable_python

:: Download Python embedded package
echo Downloading Python 3.8.10...
curl -L -o python-3.8.10-embed-amd64.zip https://www.python.org/ftp/python/3.8.10/python-3.8.10-embed-amd64.zip

:: Extract Python
echo Extracting Python...
tar -xf python-3.8.10-embed-amd64.zip

:: Fix python38._pth to enable site-packages
echo Modifying python38._pth...
echo import site>> python38._pth

:: Download get-pip.py
echo Downloading pip...
curl -L -o get-pip.py https://bootstrap.pypa.io/get-pip.py

:: Install pip
echo Installing pip...
.\python.exe get-pip.py --no-warn-script-location

:: Add Scripts directory to temporary PATH
set "PATH=%CD%\Scripts;%PATH%"

:: Install required packages
echo Installing required packages...
.\Scripts\pip.exe install --no-warn-script-location streamlit==1.32.0 pandas==2.0.3 plotly==5.18.0 openpyxl==3.1.2

:: Cleanup
del python-3.8.10-embed-amd64.zip
del get-pip.py

cd ..
echo Portable Python environment setup complete! 