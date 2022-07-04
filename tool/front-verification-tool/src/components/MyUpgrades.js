import React, { useState, useEffect } from 'react';
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';
import { Button } from 'primereact/button';
import API from '../service/API';
import Web3 from "./Web3";
import Backdrop from '@material-ui/core/Backdrop';
import CircularProgress from '@material-ui/core/CircularProgress';
import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles((theme) => ({
    backdrop: {
      zIndex: theme.zIndex.drawer + 1,
      color: '#fff',
    },
  }));

const TableDemo = () => {
    const [logs, setLogs] = useState([]);
    const [account, setAccount] = useState('');
    const [loading, setLoading] = useState(false);
    const classes=useStyles();
    
    useEffect(() => { list_upgrades(); }, []);

    useEffect(() => {
        async function fetchAccounts() {
            let web3 = await Web3();
            let accounts = await web3.eth.getAccounts();
            setAccount(accounts[0]);
          }
          fetchAccounts();
    }, []);

    const formatDate = (value) => {
        if(value.created_at ){
            let date = value.created_at.split("T");
            let day_month_year = date[0].split("-");
            let hour_minute = date[1].split(":");
            return `${day_month_year[2]}/${day_month_year[1]}/${day_month_year[0]} ${hour_minute[0]}:${hour_minute[1]}`
        }
    }


    function list_upgrades(){
        setLoading(true);
        API.get(`/listupgrades/${window.ethereum.selectedAddress}`).then(async res => {
            setLogs(res.data);
            setLoading(false);
        }).catch(error => {
            console.log("failed")
            setLoading(false);
        });
    }

    return (
        <div className="grid table-demo">

        {loading    ?   <Backdrop className={classes.backdrop} open>
                            <CircularProgress color="inherit" />
                        </Backdrop> : loading}

        <div className="col-12">    
            <div className="card">
            <h5>My Upgrades</h5>
            <DataTable value={logs} rows={5} paginator responsiveLayout="scroll">
                
                <Column field="specification_id" header="Specification ID" sortable style={{width: '20%'}}/>

                <Column field="proxy_address" header="Proxy Address" sortable style={{width: '35%'}}/>

                <Column field="created_at"  body={formatDate} header="Created At" sortable style={{width: '35%'}}/>
                        
                <Column header="Details" style={{width:'15%'}} body={() => (
                    <>
                        <Button icon="pi pi-search" type="button" className="p-button-text"/>
                    </>
                )}/>
            </DataTable>
            </div>
        </div>   
           
        </div>
    );
}

const comparisonFn = function (prevProps, nextProps) {
    return prevProps.location.pathname === nextProps.location.pathname;
};

export default React.memo(TableDemo, comparisonFn);
