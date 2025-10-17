import data from '../../exampleData.json'
import styles from './List.module.css'


function List() {
    return(
        <div className={styles.list}>
            {
                data.map(pkg => {
                    return(
                        <div className={styles.elements} key={pkg.name}> {pkg.name} : {pkg.version} </div>
                    );
                })
            }
        </div>
    );
}

export default List;
