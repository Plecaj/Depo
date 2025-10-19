import styles from'./InfoBar.module.css';
import {useContext} from "react";
import {PackagesData} from "../../App.jsx";
import { invoke } from "@tauri-apps/api/core";

function InfoBar() {

    const {path} = useContext(PackagesData);

    return(
        <div className={styles.bar}>
            Project : {path}

        </div>
    );
}

export default InfoBar;

