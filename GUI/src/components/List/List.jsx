import icon from '../../assets/delete.png'
import styles from './List.module.css'
import { PackagesData } from '../../App.jsx';
import {useContext} from "react";

function List() {

    const [packages] = useContext(PackagesData);

    return(
        <div className={styles.list}>
            {
                packages.map(pkg => {
                    return(
                        <div className={styles.elements}
                             key={pkg.name}> {pkg.name} : {pkg.version}
                            - {pkg.installed ?  "yes" : "no"}
                            <div className={styles.delete}>
                                <img src={icon} alt="delete" ></img>
                            </div>
                        </div>
                    );
                })
            }
        </div>
    );
}

export default List;
