import requests
import os
import sys
from bs4 import BeautifulSoup
import pandas as pd
import numpy as np
import json
from dotenv import load_dotenv

load_dotenv()


def convert_to_df(url):
    sys.stdout.reconfigure(encoding="utf-8")
    response = requests.get(url)
    response.encoding = "utf-8" 

    if response.status_code == 200:
        soup = BeautifulSoup(response.text, "html.parser")
    
    else:
        print("Request error")
        return None
    
    table = soup.find("table")
    if table:

        rows = table.find_all("tr")
        
        data = []
        
        for row in rows:
            cols = row.find_all("td")
            cols_data = [col.get_text(strip=True) for col in cols]
            if cols_data:
                data.append(cols_data)


        df = pd.DataFrame(data)

    
        df.columns = df.iloc[0] 
        df = df.drop(0)  
        df.reset_index(drop=True, inplace=True)

    else:
        print("Error creating dataframe")
        return None
    
    return df

def parse_data(df):
    df = df.iloc[1:, 0:9].reset_index(drop=True)

    df.columns = ["Date", "Name", "Base MMR", "Captain", "Pos1", "Pos2", "Pos3", "Pos4", "Pos5"]
    mapping = {'Yes': True, 'No': False}

    df['Captain'] = df['Captain'].replace(mapping)

    df["Name"] = df["Name"].replace(r'^\s*$', np.nan, regex=True)
    df = df.dropna(subset=["Name"])

    df["Name"] = df["Name"].str.lower()

    df["Date"] = pd.to_datetime(df["Date"])
    df = df.sort_values(["Name", "Date"], ascending=[True, False])
    df = df.drop_duplicates(subset=["Name"], keep="first").reset_index(drop=True)

    df = df.set_index(df.columns[0])



    return df

def sort_data(df, usernames):
    df = df[df["Name"].isin(usernames)]

    if len(df) != 10:
        found_names = df["Name"].to_list()
        print(usernames)
        print(found_names)

        missing = [u for u in usernames if u not in found_names]
        if len(missing) == 0:
            error_message = "Duplicate usernames entered, please try again."
        else:
            missing = list(set(missing))
            missing = ", ".join(missing)
            error_message = f"Missing the following usernames: {missing}, enter the questionnaire using your discord username using /form."
        raise ValueError(error_message)
    return df


def main(usernames):
    df = convert_to_df(os.getenv("SURVEY_URL"))
    df = parse_data(df)
    
    usernames = json.loads(usernames)
    df = sort_data(df, usernames)    
    df.to_csv("../data/input.csv", index=False)

if __name__ == "__main__":
    main(sys.argv[1])
    # main(1)