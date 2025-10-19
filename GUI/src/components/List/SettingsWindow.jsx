import styles from './SettingsWindow.module.css';
import closeIcon from "../../assets/delete.png";
import {useContext, useEffect, useState} from "react";
import {PackagesData} from "../../App.jsx";
import {invoke} from "@tauri-apps/api/core";


function AddWindow({isSettingsVisible, setIsSettingsVisible, Package}) {

    const [newConstarint, setNewConstarint] = useState("");
    const {path, fetchData} = useContext(PackagesData);


    const handleConstraintInputChange = (e)=>{
        setNewConstarint(e.target.value);
    }

    const modifyConstraint = async () => {
        if(newConstarint === ""){return}
        try{
            await invoke('modify_dependency_constraint' , {path: path ,name: Package.name, newConstraint: newConstarint});
            console.log("modify succeed " + Package.name);
            fetchData();
        }catch(e){
            console.log("error while modifying dependency constraint " + Package.name + " : "+ e);
            alert(e);
        }
    }

    const removeConstraint = async () => {
        try{
            await invoke('remove_dependency_constraint', {path: path ,name: Package.name});
            console.log("remove succeed " + Package.name);
            fetchData();
        }catch(e){
            console.log("error while remove constraint " + Package.name + " : "+ e);
            alert(e);
        }
    }

    return(
        <>
            {isSettingsVisible &&
                <div  className={styles.backGround}>
                    <div className={styles.window}>

                        <div className={styles.header}>
                            <button className={styles.closeButton} onClick={() => setIsSettingsVisible(false)}> <img src={closeIcon} alt="X"></img> </button>
                        </div>

                        <div className={styles.info}> {Package.version_constraint ? ` current constraint : ${Package.version_constraint} for ${Package.name}` : ` no version constraint`  } </div>

                        <div className={styles.input}><input value={newConstarint} onChange={handleConstraintInputChange}></input></div>

                        <div className={styles.row}>
                            <div onClick={modifyConstraint} className={styles.button}> modify </div>
                            <div onClick={removeConstraint} className={styles.button}> remove </div>
                        </div>


                    </div>
                </div>
            }
        </>
    );
}

export default AddWindow;
