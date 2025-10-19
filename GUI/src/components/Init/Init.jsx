import styles from'./Init.module.css';
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import {useContext} from "react";
import {PackagesData} from "../../App.jsx";

function Init() {

    const {setPath, setError} = useContext(PackagesData);

    async function Init(){

        let path = await open({
            multiple: false,
            directory: true,
        });

        if(!path){return}

        try{
            await invoke('init', {path: path});
            console.log("init  project :" + path);
            setPath(path);
        }catch(e){
            console.log("somthing went wrong! with init project : " + e);
            setError(e);
            alert(e);
        }

    }

    return(
        <>
            <div className={styles.button} onClick={Init} > Init Project </div>
        </>
    );
}

export default Init;

