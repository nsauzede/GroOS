import time
import os
from datetime import datetime

now = datetime.now()
# Get timezone abbreviation like CEST
tz = time.strftime('%Z')
today_text = now.strftime(f"%a %b %d %I:%M:%S %p {tz} %Y")
CURRENT_DATE = f"&laquo;{today_text}&raquo;"  # Fixed date as per query
print(f"{CURRENT_DATE=}")
