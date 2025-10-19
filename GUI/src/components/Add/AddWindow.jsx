import styles from './Add.module.css';
import { invoke } from "@tauri-apps/api/core";
import {useContext,  useState} from "react";
import {PackagesData} from "../../App.jsx";
import addIcon from "../../assets/add.png";
import searchIcon from "../../assets/search.png";
import closeIcon from "../../assets/delete.png";


function AddWindow({isVisible, setIsVisible}) {

    const {path, fetchData} = useContext(PackagesData);

    const [dependency, setDependency] = useState({});
    
    const [selectedDep, setSelectedDep] = useState("");
    const [name, setName] = useState("");
    const [version, setVersion] = useState("");

    const HandleInputChange = (e) =>{
        setName(e.target.value);
    }
    async function handleSelectChange(e){
        await setSelectedDep(e.target.value);
    }

    const handleVersionChange = (e) => {
        setVersion(e.target.value);
    }

    async function search(){
        if(name.length <= 0){return}
        try{
            let dependency = await invoke('find_dependency' , {path: path, name: name})

            console.log("found dependency :"+ JSON.stringify(dependency));
            setDependency(dependency);
        }
        catch(e){
            console.log("dependency not found : " + e);
            alert(e);
        }
    }

    async function addDependency(){
        if(selectedDep === ""){return}
        try{
            let depSelected = Object.values(dependency).find(dep => dep.name.toLowerCase() === selectedDep.toLowerCase());
            if(version !== "")
            {
                depSelected.version_constraint = version;
            }

            await invoke('add_dependency' , {path: path, dep: depSelected});
            console.log("dependency added ! with version :"  + depSelected.version_constraint);
            fetchData();
        }catch(e){
            console.log("something went wrong with adding dependency : " + e);
            alert(e);
        }
    }


    return(
        <>
            {isVisible &&
                <div  className={styles.backGround}>
                    <div className={styles.window}>

                        <div className={styles.header}>
                            <button className={styles.closeButton} onClick={() => setIsVisible(false)}> <img src={closeIcon} alt="X" ></img> </button>
                        </div>

                        <div className={styles.row}>
                            <input type="text" value={name} onChange={HandleInputChange} placeholder="name" ></input>
                            <button onClick={search} className={styles.searchButton}> <img alt="search" src={searchIcon}></img>  </button>
                        </div>

                        <div className={styles.row}>
                            <select value={selectedDep} onChange={handleSelectChange}>

                                <option value=""> ----select---  </option>
                                {Object.values(dependency).map(dep =>
                                    <option key={dep.name} value={dep.name}>
                                        {dep.name}
                                    </option>
                                )}

                            </select>
                            <input type="text" value={version} onChange={handleVersionChange} placeholder="version" ></input>
                        </div>
                        <button onClick={addDependency} className={styles.addButton} > <img alt="add" src={addIcon}></img> </button>

                    </div>
                </div>
            }
        </>
    );
}

export default AddWindow;
