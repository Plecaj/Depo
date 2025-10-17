import styles from'./Install.module.css';
import { PackagesData } from '../../App.jsx';
import {useContext} from "react";


function Install() {

    const [Package, setPackage] = useContext(PackagesData);

    const installPkg = () =>{
        setPackage( Package.map( (pkg) => {
            if(!pkg.installed){
                return {...pkg, installed: true};
            }
            else{
                return pkg;
            }
        }));
    }

    return(
        <>
            <div className={styles.button} onClick={installPkg}> Install </div>
        </>
    );
}

export default Install;

