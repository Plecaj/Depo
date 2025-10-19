import List from './components/List/List.jsx'
import ManagerBar from './components/ManagerBar/ManagerBar.jsx'
import HelpBar from "./components/HelpBar/HelpBar.jsx"
import InfoBar from "./components/InfoBar/InfoBar.jsx"

import styles from './App.module.css'
import {createContext, useEffect, useState} from "react"
import { invoke } from "@tauri-apps/api/core"

export const PackagesData = createContext(null);

function App() {

    const [path, setPath] = useState(null);
    const [packageData, setPackageData] = useState();
    const[error, setError] = useState(null);

    async function fetchData() {
        try{
            const data = await invoke('get_project_deps' , {path: path} )
            setPackageData(data);
            console.log("data fechted");
        }catch(e){
            console.log("filed to fetch data : "  + e);
            setError(e);
        }
    }
    useEffect(() => {
        if(path){
            fetchData();
        }
    },[path])

    return(
        <PackagesData.Provider value={{path , setPath, packageData, setPackageData, fetchData, error, setError}}>
            <div className={styles.app}>
                <div className={styles.HelpBar}>
                    <HelpBar/>
                </div>
                {path != null &&
                    <div className={styles.column}>
                        <div className={styles.Bar}>
                            <ManagerBar/>
                        </div>
                        <div className={styles.List}>
                            <List/>
                        </div>
                        <div className={styles.InfoBar}>
                            <InfoBar/>
                        </div>
                    </div>
                }
                {path == null &&
                    <div className={styles.noProject}> no project selected <br/>
                        {error &&  error }
                    </div>
                }

            </div>
        </PackagesData.Provider>
    );
}

export default App;
