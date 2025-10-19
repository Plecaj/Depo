import styles from'./Build.module.css';
import {useContext} from "react";
import {PackagesData} from "../../App.jsx";
import { invoke } from "@tauri-apps/api/core";

function Build() {
    const {path, setError}= useContext(PackagesData);

    async function build(){
        try{
            await invoke('build_dependencies' , {path:path});
            console.log("Building dependencies");
        }catch(e){
            console.log("filed building dependencies : " + e);
            setError(e);
            alert(e);

        }
    }

    return(
        <>
            <div className={styles.button} onClick={build} > Build </div>
        </>
    );
}

export default Build;

