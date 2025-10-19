import icon from '../../assets/delete.png'
import styles from './List.module.css'
import { PackagesData } from '../../App.jsx';
import {useContext, useEffect, useState} from "react";
import { invoke } from "@tauri-apps/api/core";
import settingsIcon from "../../assets/settings2.png";
import SettingsWindow from "./SettingsWindow.jsx";

function List() {

    const {packageData, path, fetchData} = useContext(PackagesData);
    const [isSettingsVisible, setIsSettingsVisible] = useState(false);

    async function deleteDep(name){
        try{
            await invoke('delete_dependency', {path: path, name: name});
            console.log("deleted dependency " + name);
            fetchData();
        }catch(e){
            console.log(" problem with deleting  dependency " + name + " : " + e);
            alert(e);
        }

    }

    async function updateDep(name){
        try{
            await invoke('update_dependency', {path: path, name: name});
            console.log("updated dependency " + name );
            fetchData();
        }catch(e){
            console.log(" problem with updating  dependency " + name + " : " +e);
            alert(e);
        }
    }
    const openSettings = () => {
        setIsSettingsVisible(true);
    }


    return(
        <div className={styles.list}>
            {packageData &&   Object.values(packageData).map(pkg =>
                <div className={styles.elements} key={pkg.name}>
                      {pkg.name}{pkg.version &&  `@${pkg.version}` }

                    <div className={styles.buttonBox}>
                        <div className={`${styles.button} ${styles.update}`} onClick={ ()=> updateDep(pkg.name) }>
                                update
                        </div>

                        <div className={`${styles.button} ${styles.delete}`} onClick={ ()=> deleteDep(pkg.name) }>
                            <img src={icon} alt="delete" ></img>
                        </div>

                        <div className={`${styles.button} ${styles.settings}`} onClick={openSettings}>
                            <img src={settingsIcon} alt="settings"></img>
                        </div>
                    </div>

                    {isSettingsVisible &&
                        <SettingsWindow isSettingsVisible={isSettingsVisible}  setIsSettingsVisible={setIsSettingsVisible} Package={pkg} ></SettingsWindow>
                    }
                </div>
            )}
        </div>
    );
}

export default List;
