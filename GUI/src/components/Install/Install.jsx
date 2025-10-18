import styles from'./Install.module.css';
import { PackagesData } from '../../App.jsx';
import {useContext} from "react";


function Install() {

    const installPkg = () =>{
    }

    return(
        <>
            <div className={styles.button} onClick={installPkg}> Install </div>
        </>
    );
}

export default Install;

