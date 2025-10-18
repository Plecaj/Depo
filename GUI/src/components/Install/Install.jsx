import styles from'./Install.module.css';
import { PackagesData } from '../../App.jsx';
import {useContext} from "react";
import { invoke } from "@tauri-apps/api/core";

function Install() {

    const {path} = useContext(PackagesData);

    async function installPkg() {
        try{
            await invoke('install_dependencies', {path:path});
            console.log("Install dependencies");
        }catch(e){
            console.log("problem with installing dependancy :  " + e);
        }
    }

    return(
        <>
            <div className={styles.button} onClick={installPkg}> Install </div>
        </>
    );
}

export default Install;

