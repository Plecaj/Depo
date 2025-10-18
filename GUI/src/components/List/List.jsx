import icon from '../../assets/delete.png'
import styles from './List.module.css'
import { PackagesData } from '../../App.jsx';
import {useContext, useEffect} from "react";
import { invoke } from "@tauri-apps/api/core";

function List() {

    const {packageData, path} = useContext(PackagesData);

    async function deleteDep(name){
        try{
            await invoke('delete_dependency', {path: path, name: name});
            console.log("deleted dependency " + name);
        }catch(e){
            console.log(" problem with deleting  dependency " + name + " : " + e);
        }
    }


    return(
        <div className={styles.list}>
            {packageData &&   Object.values(packageData).map(pkg => {
                return(
                    <div className={styles.elements}
                         key={pkg.name}> {pkg.name}{pkg.version_constraint &&  `@${pkg.version_constraint}` }
                        <div className={styles.delete} onClick={ ()=> deleteDep(pkg.name) }>
                            <img src={icon} alt="delete" ></img>
                        </div>
                    </div>
                );
            })}
        </div>
    );
}

export default List;
