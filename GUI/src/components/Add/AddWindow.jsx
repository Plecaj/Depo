import styles from './Add.module.css';
import { invoke } from "@tauri-apps/api/core";
import {useContext, useRef, useState} from "react";
import {PackagesData} from "../../App.jsx";

function AddWindow({isVisible, setIsVisible}) {

    const {path, fetchData} = useContext(PackagesData);

    const [dependency, setDependency] = useState({});
    const [input, setInput] = useState("");
    const [selected, setSelected] = useState("");

    const HandleInputChange = (e) =>{
        setInput(e.target.value);
    }
    const handleSelectChange = (e) =>{
        setSelected(e.target.value);
    }

    async function search(){
        if(input.length <= 0){return}
        try{
            let dependency = await invoke('find_dependency' , {path: path, name: input})
            console.log("found dependancy :"+ JSON.stringify(dependency));
            setDependency(dependency);
        }
        catch(e){
            console.log("dependancy not found : " + e);
        }
    }

    async function addDependency(){
        try{
            let depSelected = Object.values(dependency).find(dep => dep.name === selected);
            await invoke('add_dependency' , {path: path, dep: depSelected});
            console.log("dependancy added !");
            fetchData();
        }catch(e){
            console.log("somthing went wrong with adding dependancy : " + e);
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

                        <select value={selected} onChange={handleSelectChange}>
                            {Object.values(dependency).map(dep =>
                                <option key={dep.name} value={dep.name}>
                                    {dep.name}
                                </option>
                            )}
                        </select>

                        <button onClick={addDependency} > add </button>
                    </div>
                </div>
            }
        </>
    );
}

export default AddWindow;
