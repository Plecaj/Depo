import styles from'./Install.module.css';
import { PackagesData } from '../../App.jsx';
import {useContext} from "react";


function Install() {

    const [Package, setPackage] = useContext(PackagesData);

    const installPkg = () =>{
        setPackage( Package.map((pkg) => {
            return {...pkg, installed: true};
        }) );
    }

    return(
        <>
            <div className={styles.button} onClick={installPkg}> Install </div>
        </>
    );
}

export default Install;

