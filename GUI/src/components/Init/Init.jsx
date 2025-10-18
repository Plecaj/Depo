import styles from'./Init.module.css';
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import {useContext} from "react";
import {PackagesData} from "../../App.jsx";

function Init() {

    const {setPath} = useContext(PackagesData);
    const {fetchData} = useContext(PackagesData);

    async function Init(){

        let path = await open({
            multiple: false,
            directory: true,
        });

        if(!path){return}

        try{
            await invoke('init', {file: path});
            console.log("init  project :" + path);
            setPath(path);
        }catch(e){
            console.log("somthing went wrong! with init project : " + e);
        }

    }

    return(
        <>
            <div className={styles.button} onClick={Init} > init </div>
        </>
    );
}

export default Init;

