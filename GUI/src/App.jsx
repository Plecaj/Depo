import List from './components/List/List.jsx'
import ManagerBar from './components/ManagerBar/ManagerBar.jsx'
import styles from './App.module.css'
import data from './exampleData.json'
import {createContext, useState} from "react";
import HelpBar from "./components/HelpBar/HelpBar.jsx";

export const PackagesData = createContext(null);

function App() {

    const [packages, setPackages] = useState(data);

    return(
        <PackagesData.Provider value={[packages, setPackages]}>
            <div className={styles.app}>
                <div className={styles.HelpBar}>
                    <HelpBar/>
                </div>
                <div className={styles.column}>
                    <div className={styles.Bar}>
                        <ManagerBar/>
                    </div>
                    <div className={styles.List}>
                        <List/>
                    </div>
                </div>

            </div>
        </PackagesData.Provider>
    );
}

export default App;
