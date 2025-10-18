import icon from '../../assets/delete.png'
import styles from './List.module.css'
import { PackagesData } from '../../App.jsx';
import {useContext, useEffect} from "react";
import { invoke } from "@tauri-apps/api/core";

function List() {

    const {setPackageData} = useContext(PackagesData);
    const {packageData} = useContext(PackagesData);
    const {path} = useContext(PackagesData);

    return(
        <div className={styles.list}>
            {packageData &&   Object.values(packageData).map(pkg => {
                return(
                    <div className={styles.elements}
                         key={pkg.name}> {pkg.name}
                        <div className={styles.delete}>
                            <img src={icon} alt="delete" ></img>
                        </div>
                    </div>
                );
            })}
        </div>
    );
}

export default List;
