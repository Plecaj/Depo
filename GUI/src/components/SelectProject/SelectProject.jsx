import styles from'./SelectProject.module.css';
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import {useContext} from "react";
import {PackagesData} from "../../App.jsx";

function SelectProject() {

    const {setPath} = useContext(PackagesData);
    const {fetchData} = useContext(PackagesData);

    async function select(){
        let path = await open({
            multiple: false,
            directory: true,
        });

        if(!path){return}

        try{
            await invoke('get_project_deps', {path: path});
            console.log("selected project :" + path);
            setPath(path);
        }catch(e){
            console.log("somthing went wrong! with selecting project path : " + e);
        }
    }

    return(
        <>
            <div className={styles.button} onClick={select} > Select Project </div>
        </>
    );
}

export default SelectProject;

