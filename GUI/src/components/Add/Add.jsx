import AddWindow from "./AddWindow.jsx";
import styles from "./Add.module.css";
import React, {useEffect, useState} from "react";


function Add() {
    const [isWindowVisible , setIsWindowVisible] = useState(false);
    const OpenAddWindow = () => {setIsWindowVisible(true)};

    return(
        <>
            <div className={styles.button} onClick={OpenAddWindow}>
                Add
            </div>
            <AddWindow isVisible={isWindowVisible} setIsVisible={setIsWindowVisible}></AddWindow>
        </>
    );
}

export default Add;

