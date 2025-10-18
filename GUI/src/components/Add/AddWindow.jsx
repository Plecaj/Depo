import styles from './Add.module.css';
import { invoke } from "@tauri-apps/api/core";
import {useContext,  useState} from "react";
import {PackagesData} from "../../App.jsx";

function AddWindow({isVisible, setIsVisible}) {

    const {path, fetchData} = useContext(PackagesData);

    const [dependency, setDependency] = useState({});
    const [input, setInput] = useState("");
    const [selectedDep, setSelectedDep] = useState("");
    const [version, setVersion] = useState("");

    const HandleInputChange = (e) =>{
        setInput(e.target.value);
    }
    async function handleSelectChange(e){
        await setSelectedDep(e.target.value);
    }
    const handleVersionChange = (e) => {
        setVersion(e.target.value);
    }

    async function search(){
        if(input.length <= 0){return}
        try{
            let dependency = await invoke('find_dependency' , {path: path, name: input})
            console.log("found dependency :"+ JSON.stringify(dependency));
            setDependency(dependency);
        }
        catch(e){
            console.log("dependency not found : " + e);
        }
    }

    async function addDependency(){
        if(selectedDep === ""){return}
        try{
            let depSelected = Object.values(dependency).find(dep => dep.name.toLowerCase() === selectedDep.toLowerCase());
            depSelected.version_constraint = version;
            await invoke('add_dependency' , {path: path, dep: depSelected});
            console.log("dependency added ! with version :"  + depSelected.version_constraint);
            fetchData();
        }catch(e){
            console.log("something went wrong with adding dependency : " + e);
        }
    }


    return(
        <>
            {isVisible &&
                <div  className={styles.backGround}>
                    <div className={styles.window}>

                        <div className={styles.header}>
                            <button className={styles.closeButton} onClick={() => setIsVisible(false)}>X</button>
                        </div>

                        <input type="text" value={input} onChange={HandleInputChange} ></input>
                        <button onClick={search} > search </button>

                        <select value={selectedDep} onChange={handleSelectChange}>

                            <option value=""> ----select---  </option>
                            {Object.values(dependency).map(dep =>
                                <option key={dep.name} value={dep.name}>
                                    {dep.name}
                                </option>
                            )}

                        </select>

                        <input type="text" value={version} onChange={handleVersionChange} ></input>

                        <button onClick={addDependency} > add </button>
                    </div>
                </div>
            }
        </>
    );
}

export default AddWindow;
